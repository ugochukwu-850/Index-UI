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
use rocket::fs::TempFile;
use rocket::time::Instant;
use rocket::tokio::fs::File;
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
            .limit("file", 200.megabytes())
            .limit("form", 210.megabytes())
            .limit("data-form", 220.megabytes()),
    ));

    rocket::custom(figment)
        .attach(Template::fairing())
        .mount("/", routes![index, upload, download])
        .mount("/static", FileServer::from(relative!("static")).rank(1))
}

#[get("/<page>")]
fn index(page: String) -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render(page, context)
}

#[get("/download/<process_id>")]
fn download(process_id: String) -> Json<HashMap<String, Vec<String>>> {
    const EXPIRES: usize = 60 * 60 * 5;
    let mut comp = HashMap::new();
    let mut batch_index = 0;
    // loop through all the files and on failure just return
    while let Ok(stream) = get_stream(&format!("{}@{}", process_id, batch_index)) {
        // compile showing the index of the file and also the index of the row and the filename
        let ind = stream.stream_data.len();
        comp.extend(stream.stream_data);
        let _ = key_ex(&format!("{}@{}", process_id, batch_index), EXPIRES);
        batch_index += 1;
    }
    Json(comp)
}

#[post("/upload", data = "<upload>")]
async fn upload(upload: Form<Upload<'_>>) -> Json<Value> {
    //destroy the formdata
    let Upload { action, files } = &*upload;

    let (proc_id, query) = (&action.0 .0, &action.0 .1);
    println!(
        "Gotten files Len {} file action {:?}",
        files.len(),
        action.0
    );

    // get the query and search the files
    let start = Instant::now();
    let data = Mutex::new(HashMap::new());
    let filenames: Mutex<Vec<String>> = Mutex::new(Vec::new());
    let _ = match query.to_owned() {
        JsonQuery::TitleData(e) => {
            let _ = files.into_par_iter().for_each(|f| {
                if let Some(path) = f.path() {
                    if let Ok(mut excel) = open_workbook_auto(path) {
                        if let Some(x) = search::search_for_td(&mut excel, e.to_owned()) {
                            let mut data = data.lock().unwrap();

                            data.extend(x);
                            filenames
                                .lock()
                                .unwrap()
                                .push(f.name().unwrap_or("no_file_name.xlsx").to_string());
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
                            let mut data = data.lock().unwrap();
                            
                            data.extend(x);

                            filenames
                                .lock()
                                .unwrap()
                                .push(f.name().unwrap_or("no_file_name.xlsx").to_string());
                        }
                    }
                }
            });
        }
    };

    // unlock and create instance
    let data = data.lock().unwrap().to_owned();
    let filenames = filenames.lock().unwrap().to_owned();
    let stream = Stream {
        stream_id: proc_id.to_string(),
        stream_data: data,
        files: filenames,
    };

    let _ = match menu::cache::set_stream(stream) {
        Ok(e) => e,
        Err(_) => {
            return Json(
                json!({"message":"failure", "code":500, "verbose12":"An internal server error has occured. Try resending request", "type": "Database Request Set Error"}),
            )
        }
    };

    Json(json!({
        "message": "success",
        "code" : 200,
        "ex_time": (Instant::now() - start).as_seconds_f32(),
        "summary": "Batch process completed!!! ",
        "currentBatch": "index",
        "proc_id": "proc_id"
    }))
}
