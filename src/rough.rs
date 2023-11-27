use calamine::{self, open_workbook_auto_from_rs, DataType, Reader, Sheets};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufWriter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{thread, vec};

struct FileWrapper<'a>(&'a File);
fn main() {
    r_main();
}

#[test]
fn test_1() {
    const NUM: usize = 11111;
    assert_eq!(generate_files(NUM, 32, None).len(), NUM)
}

fn generate_files(amount: usize, threads: usize, path: Option<&str>) -> Vec<File> {
    // create an arc and mutex combination for data exitting and multi threading
    let files: Arc<Mutex<Vec<File>>> = Arc::new(Mutex::new(vec![]));

    // create join handle vexs for synchronization of tasks
    let mut handles = vec![];

    // create a static and multithread safe file path
    let path = match path {
        Some(e) => Arc::new(Mutex::new(e.to_string())),
        None => Arc::new(Mutex::new(
            "/home/ugochukwu/Documents/one/sample_data1.xlsx".to_string(),
        )),
    };

    // get available threads

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

pub(crate) fn r_main() {
    use calamine::{open_workbook, DataType, Reader, Xlsx};

    let started = Instant::now();
    const THREADS: usize = 200;
    let files = Arc::new(generate_files(100000, THREADS, None));
    /*println!(
        "Got all {} files in {}",
        files.len(),
        (Instant::now() - started).as_secs_f32()
    );*/
    let search_tags = Arc::new(vec![("total".to_string(), "roo".to_string())]);

    //
    let mut handles = vec![];

    let mut start = 0;
    let ex_count = files.len();
    let gap = ex_count / THREADS;
    // for each thread
    for t in 0..THREADS {
        let end = if t != ex_count - 1 {
            start + gap
        } else {
            start + gap + ex_count % THREADS
        };
        let files = Arc::clone(&files);
        let search_tags = Arc::clone(&search_tags);
        let handle = thread::spawn(move || {
            // each thread should search its list of files for the responsible coloms
            'internal_thread_loop: for s in start..end {
                /*println!(
                    "Matrix: ({}, {}), datum: {}",
                    t,
                    s,
                    files[s].metadata().unwrap().is_file()
                )*/

                // convert file to sheet type and use key tags to search the data
                let file = &files[s];
                let mut excel_workbook: Sheets<&File> =
                    match calamine::open_workbook_auto_from_rs(file) {
                        Ok(e) => e,
                        Err(e) => {
                            eprint!("An errored has occure with this files, error: {:?}", e);
                            continue 'internal_thread_loop;
                        }
                    };
                if let Some(Ok(_excel_sheet)) =
                    excel_workbook.worksheet_range(excel_workbook.sheet_names()[0].as_str())
                {
                    //let found: usize = excel_sheet.rows().flat_map(|f| {
                    //  f.iter().f
                    //});
                };
            }
        });
        handles.push(handle);
        start = end
    }

    for x in handles {
        let _ = x.join().unwrap();
    }
}

#[test]
fn r1_test() {
    r_main();
}

fn change_to_sheets() {}

pub fn search_for_td(excel: &mut Sheets<&File>, query: HashMap<String, HashSet<String>>) {
    // create a mutable ownership of excel_sheet
    // if the workbook has the work_sheet first file
    if let Some(Ok(excel_workbook)) = excel.worksheet_range(excel.sheet_names()[0].as_str()) {
        // create a map for titles matrix location and cell data
        let mut titles = HashMap::new();
        // create data map for title to map
        let mut data: HashMap<(usize, usize), Vec<String>> = HashMap::new();

        // loop through all cells
        for (row_index, col_index, cell_data) in excel_workbook.used_cells() {
            // get all the titles into the titles map filtering the non queried
            if row_index == 0 && query.contains_key(cell_data.to_string().as_str()) {
                //println!("running now");
                titles.insert((row_index, col_index), cell_data.to_string());
                continue;
            }

            // NOT TITLE and is in same coloum as one of the titles and that titles data queries includes it
            if titles.contains_key(&(0, col_index)) {
                let title = titles.get(&(0, col_index)).unwrap().to_owned();
                let cell_data = cell_data.to_string();

                // check if the title data are in them
                if query.get(&title).unwrap().contains(&cell_data) {
                    // add cell data map to the data map
                    if let Some(e) = data.get_mut(&(0, col_index)) {
                        e.push(cell_data);
                    } else {
                        data.insert((0, col_index), vec![cell_data]);
                    };
                }
            }
        }

        // map the titles to their data and//print
        let mut compr = HashMap::new();

        for (key, value) in &data {
            let key_title = titles.get(&key).unwrap();
            //println!("Key : {key_title} : Data => {value:?}");
            compr.insert(titles.get(&key).unwrap(), value);
        }
    }
}

pub fn search_for_d_x(excel: &mut Sheets<&File>, query: HashSet<String>) {
    if let Some(Ok(excel_workbook)) = excel.worksheet_range(excel.sheet_names()[0].as_str()) {
        // create a map for titles matrix location and cell data
        let mut processed_data: HashMap<String, Vec<String>> = HashMap::new();

        // loop through all cells using a iterator
        let count = excel_workbook
            .used_cells()
            .map(|(row_index, col_index, cell_data)| {
                if query.contains(&cell_data.to_string()) && row_index != 0 {
                    //println!("Found cell data for {}", cell_data.to_string());

                    // if infact this celldata has a real title
                    if let Some(title) = excel_workbook.get_value((0, col_index as u32)) {
                        if !title.is_empty() {
                            processed_data
                                .entry(title.to_string())
                                .and_modify(|e| e.push(cell_data.to_string()))
                                .or_insert(vec![cell_data.to_string()]);
                        }
                    }
                }
            })
            .count();
    }
}

pub fn search_for_d(excel: &mut Sheets<&File>, query: HashSet<String>) {
    // create a mutable ownership of excel_sheet
    // if the workbook has the work_sheet first file
    if let Some(Ok(excel_workbook)) = excel.worksheet_range(excel.sheet_names()[0].as_str()) {
        // create a map for titles matrix location and cell data
        let mut processed_data: HashMap<String, Vec<String>> = HashMap::new();

        // loop through all cells
        for (row_index, col_index, cell_data) in excel_workbook.used_cells() {
            // get all the titles into the titles map filtering the non queried

            // NOT TITLE and is in same coloum as one of the titles and that titles data queries includes it
            if query.contains(&cell_data.to_string()) && row_index != 0 {
                //println!("Found cell data for {}", cell_data.to_string());

                // if infact this celldata has a real title
                if let Some(title) = excel_workbook.get_value((0, col_index as u32)) {
                    if !title.is_empty() {
                        processed_data
                            .entry(title.to_string())
                            .and_modify(|e| e.push(cell_data.to_string()))
                            .or_insert(vec![cell_data.to_string()]);
                    }
                }
            }
        }

        //for x in processed_data {
        //println!("Key: {} :  Data => {:?}", x.0, x.1);
        //}
    }
}

pub fn search_test(data: &mut Sheets<&File>) {
    //let mut data = data;
    let cursor = data
        .worksheet_range(data.sheet_names()[0].as_str())
        .unwrap()
        .unwrap();
    let mut query = HashMap::new();
    query.insert(
        cursor.get((0, 0)).unwrap().to_string(),
        HashSet::from([cursor.get((1, 0)).unwrap().to_string()]),
    );
    query.insert(
        cursor.get((0, 1)).unwrap().to_string(),
        HashSet::from([cursor.get((1, 1)).unwrap().to_string(), "got".to_string()]),
    );
    //println!("{:?}", query);
    search_for_td(data, query);
}

pub fn search_test_d(data: &mut Sheets<&File>) {
    //let mut data = data;
    let cursor = data
        .worksheet_range(data.sheet_names()[0].as_str())
        .unwrap()
        .unwrap();
    let query: Vec<String> = cursor
        .used_cells()
        .filter(|e| e.1 % 2 == 0 && e.0 != 0)
        .map(|f| f.2.to_string())
        .collect();
    //    let query = vec!["SSDCPU".to_string()];
    //println!("{:?}", query);
    search_for_d_x(data, HashSet::from_iter(query.into_iter()));
}
