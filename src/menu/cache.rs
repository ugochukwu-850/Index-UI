use crate::Process;

use redis::{Commands, Connection, ErrorKind, RedisError, RedisResult};

use super::models::Stream;

// This module is still in build and would be optimized alot.

pub fn connection() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let connection = client.get_connection()?;
    Ok(connection)
}

pub fn get_process(id: &String) -> RedisResult<Process> {
    let mut con = connection()?;
    let pro_str: String = con.get(id)?;
    let binding: Result<Process, serde_json::Error> = serde_json::from_str(&pro_str);
    match binding {
        Ok(e) => Ok(e),
        Err(_e) => Err(RedisError::from((
            ErrorKind::TypeError,
            "Failed to parse to Process struct",
            "Serde_json parse error".to_string(),
        ))),
    }
}

pub fn set_process(pr: Process) -> RedisResult<()> {
    let mut con = connection()?;
    let value = match serde_json::to_value(&pr) {
        Ok(e) => e.to_string(),
        Err(e) => {
            return Err(RedisError::from((
                ErrorKind::TypeError,
                "Failed to parse to Process struct",
                format!("Serde_json parse error ==> {:?}", e),
            )))
        }
    };
    let x = con.set_ex(&pr.proc_id, &value, 60 * 60)?;
    Ok(x)
}

pub fn set_stream(pr: Stream) -> RedisResult<()> {
    let mut con = connection()?;
    let value = match serde_json::to_value(&pr) {
        Ok(e) => e.to_string(),
        Err(e) => {
            return Err(RedisError::from((
                ErrorKind::TypeError,
                "Failed to parse to Process struct",
                format!("Serde_json parse error ==> {:?}", e),
            )))
        }
    };
    let x = con.set_ex(&pr.stream_id, &value, 60 * 60)?;
    Ok(x)
}

pub fn get_stream(id: &String) -> RedisResult<Stream> {
    let mut con = connection()?;
    let pro_str: String = con.get(id)?;
    let binding: Result<Stream, serde_json::Error> = serde_json::from_str(&pro_str);
    match binding {
        Ok(e) => Ok(e),
        Err(_e) => Err(RedisError::from((
            ErrorKind::TypeError,
            "Failed to parse to Process struct",
            "Serde_json parse error".to_string(),
        ))),
    }
}

pub fn key_exists(key: &String) -> RedisResult<bool> {
    let mut con = connection()?;
    let ex: Result<bool, RedisError> = con.exists(key);
    match ex {
        Ok(e) => Ok(e),
        Err(f) => Err(f),
    }
}

pub fn del_key(key: &String) -> RedisResult<()> {
    let mut con = connection()?;
    // Use del to delete the entire key
    con.del(key)?;

    Ok(())
}

#[allow(unused)]
pub fn key_ex(key: &String, time: usize) -> RedisResult<()> {
    let mut con = connection()?;
    // Use del to delete the entire key
    let _ = con.expire(key, time)?;

    Ok(())
}



