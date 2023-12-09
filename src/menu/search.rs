use calamine::{self, Reader, Sheets};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::vec;

use super::excel;
use super::knubs::{cleanText, validity, filter_rows};
use super::models::IndexError;

/// Performs title + data search on all `std::io::BufReader<std::fs::File>` file types
/// Takes `excel: &mut Sheets<std::io::BufReader<std::fs::File>>`
/// Returns: `Option<HashMap<String, Vec<String>>>`
/// This operation usually takes any where from 0.0009s to 0.001s
pub fn search_for_td(
    excel: &mut Sheets<std::io::BufReader<std::fs::File>>,
    query: HashMap<String, HashSet<String>>,
    file_index: usize,
) -> Option<HashMap<String, Vec<(String, String)>>> {
    // if the workbook has the work_sheet first file
    if let Some(Ok(excel_workbook)) = excel.worksheet_range(excel.sheet_names()[0].as_str()) {
        // create a map for titles matrix location and cell data
        let mut titles = HashMap::new();

        // create data map for title to map
        let mut data: HashMap<(usize, usize), Vec<(String, String)>> = HashMap::new();

        // loop through all cells
        for (row_index, col_index, cell_data) in excel_workbook.used_cells() {
            // get all the titles into the titles map filtering the non queried
            let clean_cell_data = cleanText(&cell_data.to_string());
            if row_index == 0 && query.contains_key(&clean_cell_data) {
                //println!("found now");
                titles.insert((row_index, col_index), cell_data.to_string());
                continue;
            }

            // NOT TITLE and is in same coloum as one of the titles and that titles data queries includes it
            if titles.contains_key(&(0, col_index)) {
                let title = titles.get(&(0, col_index)).unwrap().to_owned();
                let cell_data = cell_data.to_string();

                // check if the title data are in them
                if query
                    .get(&cleanText(&title))
                    .unwrap()
                    .contains(&cleanText(&cell_data))
                {
                    // add cell data map to the data map
                    if let Some(e) = data.get_mut(&(0, col_index)) {
                        e.push((file_index.to_string(), cell_data));
                    } else {
                        data.insert((0, col_index), vec![(file_index.to_string(), cell_data)]);
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
    file_index: usize,
) -> Option<HashMap<String, Vec<(String, String)>>> {
    if let Some(Ok(excel_workbook)) = excel.worksheet_range(excel.sheet_names()[0].as_str()) {
        // create a map for titles matrix location and cell data
        let mut processed_data: HashMap<String, Vec<(String, String)>> = HashMap::new();

        // loop through all cells using a iterator
        let _count = excel_workbook
            .used_cells()
            .map(|(row_index, col_index, cell_data)| {
                let clean_cell_data = cleanText(&cell_data.to_string());
                if row_index != 0 && query.contains(&clean_cell_data) {
                    //println!("Found cell data for {}", cell_data.to_string());

                    // if infact this celldata has a real title
                    if let Some(title) = excel_workbook.get_value((0, col_index as u32)) {
                        if !title.is_empty() {
                            processed_data
                                .entry(title.to_string())
                                .and_modify(|e| {
                                    e.push((file_index.to_string(), cell_data.to_string()))
                                })
                                .or_insert(vec![(file_index.to_string(), cell_data.to_string())]);
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
            let clean_cell_data = cleanText(&cell_data.to_string());
            if query.contains(&clean_cell_data) && row_index != 0 {
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
    } else {
        None
    }
}

/// using the new adjusted algorithm
/// This function aims to return a matrix of result for a particular file <br>
/// Returns for a particular given file the (titles, matrix of body rows);
pub fn search_for_data_row(
    excel: &mut Sheets<std::io::BufReader<std::fs::File>>,
    query: HashSet<String>,
    file_name: String 
) -> Result<(Vec<String> , Vec<Vec<String>>), IndexError> {
    if let Some(excel_workbook) = excel.worksheet_range(excel.sheet_names()[0].as_str()) {
        let excel_workbook = excel_workbook?;
        let mut titles = Vec::new();
        // check if file is valid and if not return non   
        let val =  validity(&excel_workbook);
        if val.is_none() {
            return Err(IndexError::invalid_file_format("Invalid file , the row had an empty column"));
        }
        titles = val.unwrap();
        //println!("Titles before being sent to database {:?}", titles);
        let mut matrix = Vec::new();
        // search for all the rows that match the data variable and create its entire row 
        // with the filedata and append to the main list
        for (index, row) in excel_workbook.rows().enumerate() {
            if index == 0 {
                // only for the data grid ; The title is already gotten from the validation function
                continue;
            }
            let resulting_matrix = filter_rows(row, &query, file_name.to_string());
            if let Some(res) = resulting_matrix {
                // push the matrix to the main matrix page
                matrix.extend(res)
            }
        }

        //println!("Titles : {:?} Body {:?}", titles, matrix);

        return Ok((titles, matrix)) ;
    }
    Err(IndexError::not_found("The workbook was not found...? Or Could not be opened"))
}
