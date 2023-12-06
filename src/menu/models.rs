use std::collections::{HashMap, HashSet};

use rocket::{fs::TempFile, serde::json::Json};
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
pub struct  Stream {
    pub stream_id: String,
    /// key : Data Title . Values: v.0 - file index, v.1 cell data
    pub stream_data : HashMap<String, Vec<(String, String)>>,
    /// Key: File Index in batch @ batch number . Value : v.0 filename, v.1 filelastmodified
    pub files: HashMap<String, (String, String)>
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
