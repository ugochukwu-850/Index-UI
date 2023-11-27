pub (crate) mod rough;
mod comp;
#[macro_use]
extern crate rocket;
use rocket::fs::{relative, NamedFile};
use rocket::tokio::fs::File;
use rocket_dyn_templates::Template;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, upload])
        .mount("/static", FileServer::from(relative!("static")).rank(1))
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", context)
}

use std::collections::{HashMap, HashSet};

use calamine::{open_workbook, open_workbook_auto_from_rs, Sheets};
use rocket::fs::{FileServer, TempFile};
use rocket::{form::Form, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum JsonQuery {
    TitleData(HashMap<String, Vec<String>>),
    OnlyData(HashSet<String>),
    None,
}

#[derive(FromForm)]
struct Upload<'r> {
    //save: bool,
    //query: Json<JsonQuery>,
    files: Vec<TempFile<'r>>,
}
use rayon::{
    self,
    iter::{IntoParallelIterator, ParallelIterator},
};

#[post("/upload", data = "<upload>")]
async fn upload(upload: Form<Upload<'_>>) -> String {
    let files = &upload.files;
    println!("Files length {}", files.len());
    //rocket::tokio::task::spawn_blocking(|| {
        comp::u_main()
    //});
    //String::from("Yes")
}
