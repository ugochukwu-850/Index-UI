pub mod menu;
use menu::models::*;
use menu::search;

#[macro_use]
extern crate rocket;
use rocket::data::{Limits, ToByteUnit};
use rocket::fs::relative;
use rocket::time::Instant;
use rocket_dyn_templates::Template;
use serde_json::{json, Value};

use rayon::{
    self,
    iter::{IntoParallelIterator, ParallelIterator},
};
use std::collections::HashMap;
use std::sync::Mutex;

use calamine::open_workbook_auto;
use rocket::fs::FileServer;
use rocket::{form::Form, serde::json::Json};

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("port", 8080)).merge((
        "limits",
        Limits::new()
            .limit("file", 100.megabytes())
            .limit("form", 4.megabytes())
            .limit("data-form", 4.gibibytes()),
    ));

    rocket::custom(figment)
        .attach(Template::fairing())
        .mount("/", routes![index, upload])
        .mount("/static", FileServer::from(relative!("static")).rank(1))
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", context)
}

#[post("/upload", data = "<upload>")]
async fn upload(upload: Form<Upload<'_>>) -> Json<Value> {
    //destroy the formdata
    let Upload { action, files } = &*upload;
    println!("Len {}", files.len());

    // check the process action
    match action.to_owned().0 {
        Action::CreateProcess((total_batches, file_per_batch, query)) => {
            // the files must have already been unzip so just iterate through
            // create action and process first batch
            let start = Instant::now();
            let data = Mutex::new(HashMap::new());
            let _ = match query.to_owned() {
                JsonQuery::TitleData(e) => {
                    let _ = files.into_par_iter().for_each(|f| {
                        if let Some(path) = f.path() {
                            if let Ok(mut excel) = open_workbook_auto(path) {
                                if let Some(x) = search::search_for_td(&mut excel, e.to_owned()) {
                                    data.lock().unwrap().extend(x);
                                }
                            }
                        }
                    });
                }
                JsonQuery::OnlyData(e) => {
                    let _ = files.into_par_iter().for_each(|f| {
                        if let Some(path) = f.path() {
                            if let Ok(mut excel) = open_workbook_auto(path) {
                                if let Some(x) = search::search_for_d_x(&mut excel, e.to_owned()) {
                                    data.lock().unwrap().extend(x);
                                }
                            }
                        }
                    });
                }
            };
            let data = data.lock().unwrap().to_owned();
            // join all the maps together

            let proc_id = uuid::Uuid::new_v4().to_string();
            let id = proc_id.to_owned();
            let process_meta = Process {
                total_batches,
                total_files: total_batches * file_per_batch,
                current_batch: 1,
                is_complete: false,
                query,
                proc_id,
                data,
            };

            let _ = match menu::cache::set_process(process_meta) {
                Ok(e) => e,
                Err(_) => {
                    return Json(
                        json!({"message":"failure", "code":500, "verbose12":"An internal server error has occured. Try resending request", "type": "Database Request Set Error"}),
                    )
                }
            };

            // add the proc data to redis cache and return json response
            return Json(json!({
                "message": "success",
                "ex_time": (Instant::now() - start).as_seconds_f32(),
                "summary": "Process have been created!!! ",
                "proc_id": id,
            }));
        }
        Action::Stream(proc_id) => {
            // get the process id
            let Process {
                total_batches,
                total_files,
                current_batch,
                is_complete,
                query,
                proc_id,
                data,
            } = match menu::cache::get_process(&proc_id) {
                Ok(e) => e,
                Err(_) => {
                    return Json(
                        json!({"message":"failure", "code":500, "verbose12":"An internal server error has occured. Try resending request", "type": "Database Request Get Error"}),
                    )
                }
            };

            // get the query and search the files
            let start = Instant::now();
            let new_data = Mutex::new(HashMap::new());
            let _ = match query.to_owned() {
                JsonQuery::TitleData(e) => {
                    let _ = files.into_par_iter().for_each(|f| {
                        if let Some(path) = f.path() {
                            if let Ok(mut excel) = open_workbook_auto(path) {
                                if let Some(x) = search::search_for_td(&mut excel, e.to_owned()) {
                                    new_data.lock().unwrap().extend(x);
                                }
                            }
                        }
                    });
                }
                JsonQuery::OnlyData(e) => {
                    let _ = files.into_par_iter().for_each(|f| {
                        if let Some(path) = f.path() {
                            if let Ok(mut excel) = open_workbook_auto(path) {
                                if let Some(x) = search::search_for_d_x(&mut excel, e.to_owned()) {
                                    new_data.lock().unwrap().extend(x);
                                }
                            }
                        }
                    });
                }
            };
            let mut new_data = new_data.lock().unwrap().to_owned();
            new_data.extend(data.clone().into_iter());

            // join all the data together and resend to db
            let prog_process = Process {
                total_batches,
                total_files,
                current_batch: current_batch + 1,
                is_complete: if total_batches == current_batch + 1 {
                    true
                } else {
                    is_complete
                },
                query,
                proc_id,
                data: new_data,
            };

            let _ = match menu::cache::set_process(prog_process) {
                Ok(e) => e,
                Err(_) => {
                    return Json(
                        json!({"message":"failure", "code":500, "verbose12":"An internal server error has occured. Try resending request", "type": "Database Request Set Error"}),
                    )
                }
            };

            return Json(json!({
                "message": "success",
                "ex_time": (Instant::now() - start).as_seconds_f32(),
                "summary": "Batch process completed!!! ",
                "currentBatch": current_batch+1
            }));
        }
    }

    /*


    for x in files {
        println!("Name: {:?}", x.path());
        let x = calamine::open_workbook_auto(x.path().unwrap());
        println!("Opened the file  {:?}", x.is_ok());
        let mut y: Sheets<std::io::BufReader<std::fs::File>> = x.unwrap();
        if let Some(Ok(excel_workbook)) = y.worksheet_range(y.sheet_names()[0].as_str()) {
            for y in excel_workbook.used_cells() {
                println!("{:?}", y);
            }
        }
    }
    println!("Files length {}", files.len());

    //rocket::tokio::task::spawn_blocking(|| {
    //comp::u_main()
    //});
    String::from("Yes")*/
    // Json(json!({"no": null}))
}
