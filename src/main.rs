pub(crate) mod menu;
use menu::cache::del_key;
use menu::cache::get_process;
use menu::cache::get_stream;
use menu::cache::key_ex;
use menu::cache::key_exists;
use menu::excel;
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
use std::collections::HashSet;
use std::env::temp_dir;
use std::sync::Mutex;
use zip::DateTime;

use calamine::open_workbook_auto;
use rocket::fs::FileServer;
use rocket::{form::Form, serde::json::Json};

use crate::menu::excel::new_excel_file;
use crate::menu::excel::new_excel_file_t;
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
    let mut titles = Vec::new();
    let mut body = Vec::new();
    let mut batch_index = 0;
    // loop through all the files and on failure just return
    while let Ok(stream) = get_stream(&format!("{}@{}", process_id, batch_index)) {
        // compile showing the index of the file and also the index of the row and the filename
        let ind: usize = stream.batch_matrix.len();
        body.extend(stream.batch_matrix);
        titles.extend(stream.title_row);
        let _ = key_ex(&format!("{}@{}", process_id, batch_index), EXPIRES);
        batch_index += 1;
    }

    // convert the title to vec
    let titles: Vec<String> = titles.into_iter().map(|t| t).collect();
    let mut matrix = Vec::new();
    matrix.push(titles);
    matrix.extend(body);

    println!("Matrix after getting from the session \n \t {matrix:?}");
    // Create the first header row
    new_excel_file_t(matrix)
    
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
    let data = Mutex::new(Vec::new());
    let titles = Mutex::new(Vec::new());
    let _ = match query.to_owned() {
        JsonQuery::OnlyData(e) => {
            let _ = files.into_par_iter().for_each(|f| {
                if let Some(path) = f.path() {
                    if let Ok(mut excel) = open_workbook_auto(path) {
                        
                        if let Ok(file_matrix) =
                            search::search_for_data_row(&mut excel, e.to_owned())
                        {
                            println!("{:?}", file_matrix);
                            let mut data = data.lock().unwrap();
                            let mut titles = titles.lock().unwrap();

                            data.extend(file_matrix.1);
                            titles.extend(file_matrix.0)
                        }
                    }
                }
            });
        },
        _ => {

        }
    };

    // unlock and create instance
    let data = data.lock().unwrap().to_owned();
    let titles = titles.lock().unwrap().to_owned();
    println!("Before the saving \n {titles:?} \n  {data:?} \n The data lens are equal = {}", (data.len() == titles.len()));
    let stream = Stream {
        stream_id: proc_id.to_string(),
        title_row : titles,
        batch_matrix: data
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
