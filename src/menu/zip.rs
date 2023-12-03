use std::{fs::File, io::BufReader};
use rocket::fs::TempFile;
/// This file contains all functions needed for handling zips
/// 
/// Please do not edit . 


/// Unzips a zip file into a list of file buffer
/// Even though there is only one file it still returns a list

use zip::{CompressionMethod, SUPPORTED_COMPRESSION_METHODS, ZipArchive};

#[allow(unused)]
fn unzip(file: File){
    let mut zipper = ZipArchive::new(file).unwrap();
    for x in 0..zipper.len() {
        let mut file = zipper.by_index(x).unwrap();
        println!("Filename: {}", file.name());
        let mut r_file = std::io::stdout();
        let total_bytes = std::io::copy(&mut file, &mut r_file).unwrap();
        println!("")
    }
    todo!()
}

