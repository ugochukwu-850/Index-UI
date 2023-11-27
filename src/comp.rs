use std::{
    fs::{self, File},
    path::PathBuf,
    sync::Arc,
    thread,
    time::Instant, collections::HashMap,
};

use calamine::open_workbook_auto_from_rs;
use std::path::Path;
use std::sync::Mutex;
use rayon::{*, iter::{IntoParallelIterator, ParallelIterator}};

use crate::rough;

pub fn u_main() -> String {
    let start_time = Instant::now();
    //let mut handles = vec![];
    //let paths = Arc::new(paths);
    let counter = Mutex::new(0 as i32);
    let total_res: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());

    // spawn many threads
  (0..10000).into_par_iter().for_each(|t|  {
        //let end = if t != ex_count-1 {start + gap} else {start + gap + ex_count%THREADS};

        //let paths = Arc::clone(&paths);
        // for each thread
            // each thread should search its list of files for the responsible coloms

            //for path in &*paths {
            let path = Path::new("/home/ugochukwu/Documents/one/assets/sample_data1.xlsx");
            let files = &File::open(path).unwrap();
            let mut data = open_workbook_auto_from_rs(files).expect("msg");
            //println!("Search for td : Thread #{}", t);
            //rough::search_test(&path, &mut data);
            //for s in 0..1000 {
                //println!("Thread # {}",t);
                let mut num = counter.lock().unwrap();
                *num += 1;
                if let Some(x) = rough::search_test_d(&mut data) {
                    //total_res.lock().unwrap().insert("1", x);
                    total_res.lock().unwrap().extend(x.into_iter())
                }
                
            //}
            //}
        


        //let file = calamine::open_workbook_auto_from_rs(&f.file).expect("msg");
    });

    /*
    for x in handles {
        let _ = x.join();
    }

    println!(
        "Executed {} jobs in {} seconds",
        counter.lock().unwrap(),
        (Instant::now() - start_time).as_micros()
    )*/
    

    println!("Finished {} jobs  in {}", counter.lock().unwrap(), (Instant::now() - start_time).as_micros());
    println!("Printing Jobs");
    for (j, v) in total_res.lock().unwrap().to_owned() {
        println!("Key => {}, Values => {:?}", j, v);
    }
    let x = total_res.lock().unwrap().to_owned();
    let x: Vec<Vec<String>> = x.values().map(|e| {e.to_owned()}).into_iter().collect();
    let x = x.concat().concat();

    x
    
}





