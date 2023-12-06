pub(crate) mod menu;
use futures_util::SinkExt;
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
use rayon::{
    self,
    iter::{IntoParallelIterator, ParallelIterator},
};
use rocket::data::{Limits, ToByteUnit};
use rocket::fs::relative;
use rocket::fs::NamedFile;
use rocket::fs::TempFile;
use rocket::time::Instant;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncWriteExt;
use rocket_dyn_templates::Template;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env::temp_dir;
use std::sync::Mutex;
use zip::DateTime;

use calamine::open_workbook_auto;
use rocket::fs::FileServer;
use rocket::{form::Form, serde::json::Json};

use crate::menu::excel::new_excel_file;
use crate::menu::knubs::generate_index;
use crate::menu::knubs::get_bacth_index_from_proc_id;

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge((
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

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", context)
}

#[get("/download/<process_id>")]
fn download(process_id: String) -> Vec<u8> {
    const EXPIRES: usize = 60 * 5;
    let mut comp = HashMap::new();
    let mut files = HashMap::new();
    let mut batch_index = 0;
    // loop through all the files and on failure just return
    while let Ok(mut stream) = get_stream(&format!("{}@{}", process_id, batch_index)) {
        // compile showing the index of the file and also the index of the row and the filename
        let ind: usize = stream.stream_data.len();
        comp.extend(stream.stream_data);
        files.extend(stream.files);
        let _ = key_ex(&format!("{}@{}", process_id, batch_index), EXPIRES);
        batch_index += 1;
    }

    // Create the first header row
    let title_len = comp.keys().len();
    let mut totalled = vec![];
    let mut header_row: Vec<String> = vec![
        "Last Revised | Searched",
        "Filename Number",
        "Serial_number",
        "File + Serial Number",
        "Filename",
    ]
    .into_iter()
    .map(|f| f.to_string())
    .collect();
    let mut titles_list = vec!["".to_string(); title_len];
    header_row.append(&mut titles_list);
    totalled.push(header_row);
    //let file_virtual_index_map = HashMap::new();

    // loop through the files creating a list of rows
    for (index, file_data) in files.to_owned().into_iter().enumerate() {
        let mut ready_info = vec![
            "Get Time".to_string(),
            index.to_string(),
            file_data.0.to_owned(),
            format!("{} at {}", file_data.0, index),
            file_data.1.0,
        ];
        ready_info.append(&mut vec!["".to_string(); title_len]);
        totalled.push(ready_info);
    }

    // convert to excel file
    new_excel_file(totalled)
    
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
    let filenames = Mutex::new(HashMap::new());
    let bacth_index = get_bacth_index_from_proc_id(&proc_id);
    let _ = match query.to_owned() {
        JsonQuery::TitleData(e) => {
            let _ = files.into_par_iter().for_each(|f| {
                if let Some(path) = f.path() {
                    if let Ok(mut excel) = open_workbook_auto(path) {
                        let mut filenames = filenames.lock().unwrap();
                        let file_index = filenames.len() - 1;
                        let filenames_data = HashMap::from([(
                            generate_index(bacth_index, file_index),
                            (
                                f.name().unwrap_or("no_file_name.xlsx").to_string(),
                                "t".to_string(),
                            ),
                        )]);
                        filenames.extend(filenames_data);
                        if let Some(x) = search::search_for_td(&mut excel, e.to_owned(), file_index)
                        {
                            let mut data = data.lock().unwrap();

                            data.extend(x);
                        }
                    }
                }
            });
        }
        JsonQuery::OnlyData(e) => {
            let _ = files.into_par_iter().for_each(|f| {
                if let Some(path) = f.path() {
                    if let Ok(mut excel) = open_workbook_auto(path) {
                        let mut filenames = filenames.lock().unwrap();
                        let file_index = filenames.len() - 1;
                        let filenames_data = HashMap::from([(
                            generate_index(bacth_index, file_index),
                            (
                                f.name().unwrap_or("no_file_name.xlsx").to_string(),
                                "t".to_string(),
                            ),
                        )]);
                        filenames.extend(filenames_data);

                        if let Some(x) =
                            search::search_for_d_x(&mut excel, e.to_owned(), file_index)
                        {
                            let mut data = data.lock().unwrap();

                            data.extend(x);
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

    println!(
        "--> Finished processing batch {}. Execution Time : {} seconds \n",
        action.0 .0,
        (Instant::now() - start).as_seconds_f64()
    );

    Json(json!({
        "message": "success",
        "code" : 200,
        "ex_time": (Instant::now() - start).as_seconds_f32(),
        "summary": "Batch process completed!!! ",
        "currentBatch": "index",
        "proc_id": "proc_id"
    }))
}
