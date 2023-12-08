use std::collections::{HashMap, HashSet};

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
    pub query: JsonQuery
}

impl JsonQuery {
    // generates a format struct for all possible formats for 
    // a query type
    pub fn gen_format(&self) -> Vec<Format> {
        match self {
            JsonQuery::OnlyData(_) => {todo!()}
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

    #[error("Shuttle runtime error: {0}")]
    ShuttleRuntimeError(#[from] shuttle_runtime::Error),

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
