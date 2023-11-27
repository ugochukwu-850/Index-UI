use std::{
    fs::{self, File},
    path::PathBuf,
    sync::Arc,
    thread,
    time::Instant,
};

use calamine::open_workbook_auto_from_rs;
use std::path::Path;
use std::sync::Mutex;
mod rough;
use rayon::*;

fn main() {
    let start_time = Instant::now();
    let mut handles = vec![];
    //let paths = Arc::new(paths);
    let counter = Arc::new(Mutex::new(0 as i32));


    // spawn many threads
    for t in 0..40 {
        //let end = if t != ex_count-1 {start + gap} else {start + gap + ex_count%THREADS};

        //let paths = Arc::clone(&paths);
        let counter = Arc::clone(&counter);
        // for each thread
        let handle = thread::spawn(move || {
            // each thread should search its list of files for the responsible coloms

            //for path in &*paths {
            let path = Path::new("/home/ugochukwu/Documents/one/assets/sample_data1.xlsx");
            let files = &File::open(path).unwrap();
            let mut data = open_workbook_auto_from_rs(files).expect("msg");
            let counter = Arc::clone(&counter);
            //println!("Search for td : Thread #{}", t);
            //rough::search_test(&path, &mut data);
            //for s in 0..1000 {
                //println!("Thread # {} inner: {}",t, s);
                let mut num = counter.lock().unwrap();
                *num += 1;
                rough::search_test_d(&mut data);
            //}
            //}
        });

        handles.push(handle);

        //let file = calamine::open_workbook_auto_from_rs(&f.file).expect("msg");
    }

    for x in handles {
        let _ = x.join();
    }

    println!(
        "Executed {} jobs in {} seconds",
        counter.lock().unwrap(),
        (Instant::now() - start_time).as_micros()
    );
}

#[macro_use]
extern crate rocket;



