pub(crate) mod menu;
use menu::cache::del_key;
use menu::cache::get_batch;
use menu::cache::get_process;
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
use rust_xlsxwriter::Workbook;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env::temp_dir;
use std::fmt::format;
use std::sync::Arc;
use std::sync::Mutex;
use zip::DateTime;

use calamine::open_workbook_auto;
use rocket::fs::FileServer;
use rocket::{form::Form, serde::json::Json};

use crate::menu::excel::new_excel_file;
use crate::menu::excel::new_excel_file_t;
use crate::menu::knubs::file_index_gen;
use crate::menu::knubs::gen_format;
use crate::menu::knubs::generate_index;
use crate::menu::knubs::get_bacth_index_from_proc_id;
use crate::menu::knubs::get_file_trail;
use crate::menu::knubs::merge_titles;

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

    // loop through all the files and on failure just return
    let mut batch_index = 0;
    let mut titles_map: HashMap<String, usize> = HashMap::new();
    let mut file_index_map: HashMap<String, usize> = HashMap::new();
    let mut workbook = Workbook::new();
    let current_sheet = workbook.add_worksheet();
    let mut current_row = 1;
    // the file + number is going to be files number and then the index of the amount of files found in it 
    // let current format: Format;
    while let Ok(Batch {
        batch_id,
        file_results,
        query,
    }) = get_batch(&format!("{}@{}", process_id, batch_index))
    {
       

        // loop through every file processing its rows

        file_results.into_iter().for_each(
            |FileResult {
                 titles,
                 body_matrix,
             }| {
                // process the title
                let title_index_map =
                    match merge_titles(titles, &mut titles_map, current_sheet, &query) {
                        Ok(e) => e,
                        Err(_) => return,
                    };
                // println!("\n\n\n Title Index Map: {title_index_map:?}");
                // now process each file row

                for mut row in body_matrix {
                    // set the row index and file index variable 
                    let file_map_match_index = row.pop().unwrap();
                    let file_index = file_index_gen(&mut file_index_map, &row[4]);
                    let serial_file_index = format!("{}-{}", file_index, file_map_match_index);
                    let arb = vec![file_index.to_string(), current_row.to_string(), serial_file_index];
                    //let query = &query.to_owned();
                    // write each value based on its formatting
                    // for now : No formatting
                    // println!("Each row: {row:?}");
                    for (index, mut cell) in row.into_iter().enumerate() {
                        if (1..=3).contains(&index) {
                            cell = arb[index-1].to_owned();
                        }
                        let format = gen_format(&query, index, current_row);
                        _ = current_sheet.write_with_format(current_row as u32, title_index_map[index] as u16, cell, &format);
                    }

                    current_row += 1;
                }
            },
        );

        batch_index += 1;
        _ = key_ex(&batch_id, EXPIRES)
    }
    current_sheet.autofit();
    workbook.save_to_buffer().unwrap().to_vec()
    
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
    let batch = Mutex::new(Batch::new(proc_id.to_string(), query.to_owned()));
    let failed_instances = Mutex::new(Vec::new());
    let _ = match query.to_owned() {
        JsonQuery::OnlyData(e) => {
            let _ = files.into_par_iter().for_each(|f| {
                // if file does even have a real path
                if let Some(path) = f.path() {
                    // if the file could be opened
                    match open_workbook_auto(path) {
                        // on success
                        Ok(mut excel) => {
                            // search for the info
                            match search::search_for_data_row(
                                &mut excel,
                                e.to_owned(),
                                get_file_trail(f.raw_name())
                            ) {
                                // if the search was successful
                                Ok(file_matrix) => {
                                    // println!("Gotten files => Len == {:?}", file_matrix.0.len());

                                    // create a fileresult instance
                                    let file_result = FileResult {
                                        titles: file_matrix.0,
                                        body_matrix: file_matrix.1,
                                    };

                                    // update the batch struct with the file result
                                    let mut batch = batch.lock().unwrap();
                                    batch.file_results.push(file_result);

                                }

                                // log an error with the filename and the error reason
                                // clean error handling already provides a good log message
                                Err(e) => {
                                    let mut failed = failed_instances.lock().unwrap();
                                    failed.push((f.name().unwrap(), e.to_string()))
                                }
                            }
                        }
                        Err(e) => {
                            let mut failed = failed_instances.lock().unwrap();
                            failed.push((f.name().unwrap(), e.to_string()))
                        }
                    }
                    // Add the file to failed instances
                } else {
                    let mut failed = failed_instances.lock().unwrap();
                    failed.push((f.name().unwrap(), "failed to open path".to_string()))
                }
            });
        }
        JsonQuery::TitleData(e) => {
            let _ = files.into_par_iter().for_each(|f| {
                // if file does even have a real path
                if let Some(path) = f.path() {
                    // if the file could be opened
                    match open_workbook_auto(path) {
                        // on success
                        Ok(mut excel) => {
                            // search for the info
                            match search::search_for_data_row_1(
                                &mut excel,
                                e.to_owned(),
                                get_file_trail(f.raw_name())
                            ) {
                                // if the search was successful
                                Ok(file_matrix) => {
                                    // println!("Gotten files => Len == {:?}", file_matrix.0.len());

                                    // create a fileresult instance
                                    let file_result = FileResult {
                                        titles: file_matrix.0,
                                        body_matrix: file_matrix.1,
                                    };

                                    // update the batch struct with the file result
                                    let mut batch = batch.lock().unwrap();
                                    batch.file_results.push(file_result);

                                }

                                // log an error with the filename and the error reason
                                // clean error handling already provides a good log message
                                Err(e) => {
                                    let mut failed = failed_instances.lock().unwrap();
                                    failed.push((f.name().unwrap(), e.to_string()))
                                }
                            }
                        }
                        Err(e) => {
                            let mut failed = failed_instances.lock().unwrap();
                            failed.push((f.name().unwrap(), e.to_string()))
                        }
                    }
                    // Add the file to failed instances
                } else {
                    let mut failed = failed_instances.lock().unwrap();
                    failed.push((f.name().unwrap(), "failed to open path".to_string()))
                }
            });
        }
    };

    let batch = batch.lock().unwrap().to_owned();

    let _ = match menu::cache::set_stream(batch) {
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
        "proc_id": "proc_id",
        "failed_instances": failed_instances.lock().unwrap().to_owned()
    }))
}
