use std::collections::{HashMap, HashSet};

use chrono::{NaiveDate, Utc};
use rocket::{fs::TempFile, serde::json::Json};
use rust_xlsxwriter::Format;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JsonQuery {
    TitleData(HashMap<String, HashSet<String>>),
    OnlyData(HashSet<String>),
}

#[derive(FromForm)]
pub struct Upload<'r> {
    pub action: Json<(String, JsonQuery)>,
    //query: Json<JsonQuery>,
    pub files: Vec<TempFile<'r>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Action {
    Stream((String, JsonQuery)),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stream {
    pub stream_id: String,
    pub title_row: Vec<String>,
    /// key : Data Title . Values: v.0 - file index, v.1 cell data
    pub batch_matrix: Vec<Vec<String>>,
    // Key: File Index in batch @ batch number . Value : v.0 filename, v.1 filelastmodified
    //pub files: HashMap<String, (String, String)>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Batch {
    pub batch_id: String,
    pub file_results: Vec<FileResult>,
    pub query: JsonQuery,
}

impl JsonQuery {
    // generates a format struct for all possible formats for
    // a query type
    pub fn gen_format(&self) -> Vec<Format> {
        match self {
            JsonQuery::OnlyData(_) => {
                todo!()
            }
            JsonQuery::TitleData(_) => todo!(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileResult {
    pub titles: Vec<String>,
    pub body_matrix: Vec<Vec<String>>,
}

impl FileResult {
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
            body_matrix: Vec::new(),
        }
    }
}

impl Batch {
    pub fn new(batch_id: String, query: JsonQuery) -> Self {
        Self {
            batch_id,
            file_results: Vec::new(),
            query,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Process {
    pub total_batches: u64,
    // a uuid string
    pub proc_id: String,
    pub data: HashMap<String, Vec<String>>,
}
impl Process {
    pub fn new() -> Self {
        Self {
            total_batches: 1,
            proc_id: uuid::Uuid::new_v4().to_string(),
            data: HashMap::new(),
        }
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("Rocket error: {0}")]
    RocketError(#[from] rocket::Error),

    #[error("Calamine error: {0}")]
    CalamineError(#[from] calamine::Error),

    #[error("Log error: {0}")]
    LogError(#[from] log::SetLoggerError),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("UUID error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Xlsxwriter error: {0}")]
    XlsxwriterError(#[from] rust_xlsxwriter::XlsxError),

    #[error("File format error: {0}")]
    FileFormatError(String),

    #[error("No Match was found: {0}")]
    NoMatchFound(String),

    #[error("No Match was found: {0}")]
    NotFound(String),
}

impl IndexError {
    pub fn invalid_file_format<T: ToString>(msg: T) -> IndexError {
        IndexError::FileFormatError(msg.to_string())
    }

    pub fn no_match_found<T: ToString>(msg: T) -> IndexError {
        IndexError::NoMatchFound(msg.to_string())
    }

    /// Addresses anywhere option nones where returned
    pub fn not_found<T: ToString>(msg: T) -> IndexError {
        IndexError::NotFound(msg.to_string())
    }
}

// destino
use crate::schema::destino_users;
use diesel::prelude::*;


#[derive(Debug, Clone, Serialize, PartialEq, Queryable, Deserialize, Insertable)]
#[diesel(table_name = destino_users)]
pub struct Duserreq {
    // Specify the id column as the primary key
    pub id: uuid::Uuid,
    pub fullname: String,
    pub email: String,
    pub phone_number: i64, // Assuming you're using Diesel's Text type
    pub joined: NaiveDate,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct DuserInst {
    pub fullname: String,
    pub email: String,
    pub phone_number: i64, // Assuming you're using Diesel's Text type

}

impl Into<Duserreq> for DuserInst {
    fn into(self) -> Duserreq {
        // Call the method defined above to perform the conversion
        let DuserInst { fullname, email, phone_number } = self;
        Duserreq { id: uuid::Uuid::new_v4(), fullname, email, phone_number, joined: chrono::Utc::now().date_naive() }
    }
}

impl Duserreq {
    fn verify_email(email: &String) -> bool {
        use regex::Regex;
        // Define the regex pattern for email validation
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    
        // Check if the email matches the regex pattern
        re.is_match(email)
    }

    pub fn validate_and_save(&mut self) -> Option<Self> {
        self.id = uuid::Uuid::new_v4();
        self.joined = chrono::Utc::now().date_naive();
        if self.phone_number.to_string().len() < 9 || !Self::verify_email(&self.email) || self.fullname.len() < 4{
            return None;
        }
       Some( crate::menu::db::cursor::create_d_user(self))
    }
}