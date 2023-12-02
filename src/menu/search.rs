use calamine::{self, Reader, Sheets};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::vec;

/// Performs title + data search on all `std::io::BufReader<std::fs::File>` file types
/// Takes `excel: &mut Sheets<std::io::BufReader<std::fs::File>>`
/// Returns: `Option<HashMap<String, Vec<String>>>`
/// This operation usually takes any where from 0.0009s to 0.001s
pub fn search_for_td(
    excel: &mut Sheets<std::io::BufReader<std::fs::File>>,
    query: HashMap<String, HashSet<String>>,
) -> Option<HashMap<String, Vec<String>>> {
    // if the workbook has the work_sheet first file
    if let Some(Ok(excel_workbook)) = excel.worksheet_range(excel.sheet_names()[0].as_str()) {
        // create a map for titles matrix location and cell data
        let mut titles = HashMap::new();

        // create data map for title to map
        let mut data: HashMap<(usize, usize), Vec<String>> = HashMap::new();

        // loop through all cells
        for (row_index, col_index, cell_data) in excel_workbook.used_cells() {
            // get all the titles into the titles map filtering the non queried
            if row_index == 0 {
                //println!("cell data {col_index} - {:?} - {} -> {:?}", cell_data.to_string().replace("\r", ""), cell_data.to_string().contains("\n"), query.get(&cell_data.to_string()));
            }
            if row_index == 0 && query.contains_key(&cell_data.to_string()) {
                //println!("found now");
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
            let _key_title = titles.get(&key).unwrap();
            //println!("Key : {key_title} : Data => {value:?}");
            compr.insert(titles.get(&key).unwrap().to_owned(), value.to_owned());
        }
        return Some(compr);
    } else {
        return None;
    }
}

/// Performs Data Only search on all `std::io::BufReader<std::fs::File>` file types
/// Takes `excel: &mut Sheets<std::io::BufReader<std::fs::File>>` and a query type of `HashSet<String>`
/// 
/// Returns: `Option<HashMap<String, Vec<String>>>`
/// This operation usually takes any where from 0.0009s to 0.001s
/// Also uses an iter instead of conditional `for loop`
pub fn search_for_d_x(
    excel: &mut Sheets<std::io::BufReader<std::fs::File>>,
    query: HashSet<String>,
) -> Option<HashMap<String, Vec<String>>> {
    if let Some(Ok(excel_workbook)) = excel.worksheet_range(excel.sheet_names()[0].as_str()) {
        // create a map for titles matrix location and cell data
        let mut processed_data: HashMap<String, Vec<String>> = HashMap::new();
      
        // loop through all cells using a iterator
        let _count = excel_workbook
            .used_cells()
            .map(|(row_index, col_index, cell_data)| {
                if row_index != 0 && query.contains(&cell_data.to_string()) {
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
        return Some(processed_data);
    }

    None
}



#[allow(unused)]
/// Performs Data Only search on all `std::fs::File` file types
/// Takes `excel: &mut Sheets<std::fs::File>` and a query type of `HashSet<String>`
/// 
/// Returns: `Option<HashMap<String, Vec<String>>>`
/// This operation usually takes any where from 0.0009s to 0.001s
/// Also uses an iter instead of conditional `for loop`

pub fn search_for_d(
    excel: &mut Sheets<&File>,
    query: HashSet<String>,
) -> Option<HashMap<String, Vec<String>>> {
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
        Some(processed_data)
    }
    else {
        None
    }
}
