use std::{
    fs::{self, File},
    path::PathBuf,
    sync::Arc,
    thread,
    time::Instant,
};

use calamine::open_workbook_auto_from_rs;

mod rough;

fn main() {
    let start_time = Instant::now();
    println!("Running search_test");
    let dir_path = "/home/ugochukwu/Documents/one/assets";
    let paths = match fs::read_dir(dir_path) {
        Ok(e) => e
            .filter(|f| {
                if let Ok(r) = f {
                    println!(
                        "Found paths: {:?}",
                        r.path().canonicalize().unwrap().extension()
                    );
                    r.path().extension().unwrap() == "xlsx"
                } else {
                    false
                }
            })
            .map(|f| f.unwrap().path())
            .collect::<Vec<PathBuf>>(),
        Err(e) => {
            panic!("Failed to open file")
        }
    };

    const THREADS: usize = 200;
    let mut handles = vec![];
    let paths = Arc::new(paths);
    //println!("{:?}", file_queries);
    //let ex_count = 10000;

    //let mut start = 0;
    //let gap = ex_count/THREADS;

    // spawn many threads
    for t in 0..30 {
        //let end = if t != ex_count-1 {start + gap} else {start + gap + ex_count%THREADS};

        let paths = Arc::clone(&paths);
        // for each thread
        let handle = thread::spawn(move || {
            // each thread should search its list of files for the responsible coloms

            for path in &*paths {
                let files = &File::open(path).unwrap();
                let mut data = open_workbook_auto_from_rs(files).expect("msg");
                //println!("Search for td : Thread #{}", t);
                //rough::search_test(&path, &mut data);

                for s in 0..15 {
                    //println!("Thread # {} inner: {}",t, s);
                    rough::search_test(&mut data);
                }
            }
        });

        handles.push(handle);

        //let file = calamine::open_workbook_auto_from_rs(&f.file).expect("msg");
    }

    for x in handles {
        let _ = x.join();
    }

    println!("Finished in {}", (Instant::now() - start_time).as_secs());
}
