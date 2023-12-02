pub(crate) mod menu;
use menu::cache::del_key;
use menu::cache::get_process;
use menu::cache::get_stream;
use menu::cache::key_ex;
use menu::cache::key_exists;
use menu::models::*;
use menu::search;
mod tests;

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
            .limit("file", 5.megabytes())
            .limit("form", 210.megabytes())
            .limit("data-form", 220.megabytes()),
    ));

    rocket::custom(figment)
        .attach(Template::fairing())
        .mount("/", routes![index, upload, download])
        .mount("/static", FileServer::from(relative!("static")).rank(1))
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", context)
}

#[get("/download/<process_id>")]
fn download(process_id: String) -> Json<HashMap<String , Vec<String>>> {
    const EXPIRES : usize = 60 * 60 * 5;
    if let Ok(exists) = key_exists(&process_id) {
        if exists {
            if let Ok(Process {
                total_batches,
                mut data,
                ..
            }) = get_process(&process_id)
            {
                for x in 1..total_batches+1 {
                    // get t he stream of that batch
                    if let Ok(Stream {
                        stream_data, stream_id
                    }) = get_stream(&format!("{}@{x}", process_id))
                    {
                        //delete the stream id 
                        let _ = key_ex(&stream_id, EXPIRES);
                        data.extend(stream_data.to_owned().into_iter())
                    }
                }

                // now compile this main file into an excel file and return

                // but for this testing purpose just return the Map as string
                //delete the key
                let _ = key_ex(&process_id, EXPIRES);
                return Json(data);

            }
        }
    }
    Json(HashMap::new())
}

#[post("/upload", data = "<upload>")]
async fn upload(upload: Form<Upload<'_>>) -> Json<Value> {
    //destroy the formdata
    let Upload { action, files } = &*upload;
    println!("Gotten files Len {} file action {:?}", files.len(), action.0);

    // check the process action
    match action.to_owned().0 {
        Action::CreateProcess((total_batches, query)) => {
            // the files must have already been unzip so just iterate through
            // create action and process first batch
            let start = Instant::now();

            // create a HashMap container
            let data = Mutex::new(HashMap::new());

            // Process based on query type
            let _ = match query.to_owned() {
                // If TitleData  query
                JsonQuery::TitleData(e) => {
                    // Iter through the files processing and updating the comp map
                    
                    let _ = (0..files.len()).into_par_iter().for_each(|index| {
                        let f = &files[index];
                        // if file has a path not None
                        if let Some(path) = f.path() {
                            // if the excel_work book could be created
                            if let Ok(mut excel) = open_workbook_auto(path) {
                                // if the search in the file gave a valid response
                                if let Some(x) = search::search_for_td(&mut excel, e.to_owned()) {
                                    // add the response to data
                                    
                                    data.lock().unwrap().extend(x);
                                }
                            }
                        }
                    });
                }

                // If Data Only  query
                JsonQuery::OnlyData(e) => {
                    // Iter through the files processing and updating the comp map

                    let _ = (0..files.len()).into_par_iter().for_each(|index| {
                        let f = &files[index];
                        // if the excel_work book could be created

                        if let Some(path) = f.path() {
                            // if the search in the file gave a valid response

                            if let Ok(mut excel) = open_workbook_auto(path) {
                                if let Some(x) = search::search_for_d_x(&mut excel, e.to_owned()) {
                                    let names: Vec<&str> = files.iter().filter_map(|e| e.name()).collect();
                                    data.lock().unwrap().extend(x);
                                    
                                }
                            }
                        }
                    });
                }
            };

            // unlock the mutex
            let data = data.lock().unwrap().to_owned();

            // create a process instance
            let proc_id = uuid::Uuid::new_v4().to_string();
            let id = proc_id.to_owned();
            let process_meta = Process {
                total_batches,
                proc_id,
                data,
            };

            // if the process was successfully sent into the database
            let _ = match menu::cache::set_process(process_meta) {
                Ok(e) => e,
                Err(_) => {
                    // try again
                    return Json(
                        json!({"message":"failure", "code":500, "verbose12":"An internal server error has occured. Try resending request", "type": "Database Request Set Error"}),
                    );
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
        Action::Stream((proc_id, query, index)) => {
            // get the process id

            // get the query and search the files
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

            let stream = Stream {
                stream_id: format!("{proc_id}@{index}"),
                stream_data: data,
            };

            let _ = match menu::cache::set_stream(stream) {
                Ok(e) => e,
                Err(_) => {
                    return Json(
                        json!({"message":"failure", "code":500, "verbose12":"An internal server error has occured. Try resending request", "type": "Database Request Set Error"}),
                    )
                }
            };

            return Json(json!({
                "message": "success",
                "code" : 200,
                "ex_time": (Instant::now() - start).as_seconds_f32(),
                "summary": "Batch process completed!!! ",
                "currentBatch": index,
                "proc_id": proc_id
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
