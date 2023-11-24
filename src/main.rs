use calamine::{self, Sheets};
use std::fs::File;
use std::io::{BufRead, BufWriter};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, vec};

struct FileWrapper<'a>(&'a File);
fn main() {}

#[test]
fn test_1() {
    const NUM: usize = 11111;
    assert_eq!(generate_files(NUM, None).len(), NUM)
}

fn generate_files(amount: usize, path: Option<&str>) -> Vec<File> {
    // create an arc and mutex combination for data exitting and multi threading
    let files: Arc<Mutex<Vec<File>>> = Arc::new(Mutex::new(vec![]));

    // create join handle vexs for synchronization of tasks
    let mut handles = vec![];

    // create a static and multithread safe file path
    let path = match path {
        Some(e) => Arc::new(Mutex::new(e.to_string())),
        None => Arc::new(Mutex::new(
            "/home/ugochukwu/Documents/one/udoka.png"
                .to_string(),
        )),
    };

    // get available threads
    let x = thread::available_parallelism().unwrap().get();
    let threads = 32;

    // for each thread
    for t in 0..threads {
        // create MT/IO safe Vec of files
        let files = Arc::clone(&files);

        // clone path because of ealier move statement
        let path = Arc::clone(&path);

        // threads
        let handle = thread::spawn(move || {
            // get the safest estimate for each thread task and assign the rest to the last thread
            let range = if t != threads - 1 {
                0..amount / threads
            } else {
                0..(amount / threads) + amount % threads
            };

            // for each range of thread task
            for i in range {
                // unlock files vector
                let mut files = files.lock().unwrap();
                // unlock file path
                let path = path.lock().unwrap().clone().to_string();
                // read new file and push to MI/IO safe vector
                match File::open(&path) {
                    Ok(file_new) => files.push(file_new),
                    Err(err) => eprintln!("Error opening file: {}", err),
                }
            }
        });

        // tag all thread handles
        handles.push(handle);
    }

    // join all thread handles with main
    for handle in handles {
        handle.join().unwrap();
    }
    // swap files as Non copy type and return the files vector
    let mut files = files.lock().unwrap();
    std::mem::replace(&mut *files, Vec::new())
}

fn r_main() {
    use calamine::{Reader, open_workbook, Xlsx, DataType};

    let started = Instant::now();
    let files = Arc::new(generate_files(32, None));
    println!("Got all {} files in {}", files.len(), (Instant::now() - started).as_secs_f32());
    let search_tags = Arc::new(vec![("total".to_string(), "roo".to_string())]);

    //
    let mut handles = vec![];

    let threads = 32;
    let mut start = 0;
    let ex_count = files.len();
    let gap = ex_count/threads;
    // for each thread
    for t in 0..threads {
        let end = if t == ex_count-1 {start + gap} else {start + gap + ex_count%threads};
        let files = Arc::clone(&files);
        let search_tags = Arc::clone(&search_tags);
        let handle = thread::spawn(move || {
            // each thread should search its list of files for the responsible coloms
            for s in start..end {
                println!("Matrix: ({}, {}), datum: {}", t, s, files[s].metadata().unwrap().is_file());

                // convert file to sheet type and use key tags to search the data
                let file = &files[s];
                let mut excel_workbook = calamine::open_workbook_auto_from_rs(file).expect("Failed to load file");
                if let Some(Ok(excel_sheet))  = excel_workbook.worksheet_range(excel_workbook.sheet_names()[0].as_str()){
                    let found: usize = excel_sheet.rows().flat_map(|f| {
                        f.iter().f
                    });
                };
            }
        });
        handles.push(handle);
        start = end
    }
}

#[test]
fn r1_test() {
    r_main();
}

fn change_to_sheets() {}
