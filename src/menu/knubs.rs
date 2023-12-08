use std::collections::HashSet;

use calamine::{DataType, Range};

pub fn cleanText(text: &String) -> String {
    let result: String = text.chars().filter(|c| !c.is_whitespace()).collect();

    result
}

pub fn get_bacth_index_from_proc_id(proc_id: &String) -> usize {
    let id = proc_id.split_once("@").unwrap();
    id.1.to_string().parse::<usize>().unwrap()
}

pub fn generate_index(batch_index: usize, index: usize) -> String {
    format!("{}@{}", batch_index, index)
}
/// takes a range of cells and checks that they pass the validity test
/// If they pass it returns the titles
pub fn validity(excel: &Range<DataType>) -> Option<Vec<String>> {
    // No empty first row
    for (index, row) in excel.rows().enumerate() {
        // if the title cell is empty : Past first row break
        if index > 0 {
            break;
        }
        let mut titles: Vec<String> = vec![
            "Filename",
            "File last Revised",
            "Serial Number",
            "File Index",
            "file Index + Serial Number",
            "Data",
        ]
        .into_iter()
        .map(|f| f.to_string())
        .collect();

        // if update the titles row
        for cell in row {
            if cell.is_empty() {
                return None;
            }
            titles.push(cell.to_string());
        }

        // return the titles
        return Some(titles);
    }

    None
}

/// Creates N amount of row instances each time a query Data is found
/// Returns a matrix of rows for each query instance being found <br>
/// Examples
/// ```rust
/// let example_response = vec!["name_of_query_found_in_row", "remaining row data"];
/// println!("This would be an example of a response");
/// ```
pub fn filter_rows(row: &[DataType], query: &HashSet<String>, file_name: String) -> Option<Vec<Vec<String>>> {
    // if the row has any of the query then create a row instance with the row
    let rows: Vec<String> = row.iter().map(|d| cleanText(&d.to_string())).collect();
    let mut gotten_row_matches = Vec::new();
    
    for data in query {
        println!("Data searching for {data}");
        if rows.contains(data) {
            let mut new_row: Vec<String> = vec![
                &file_name,
                "last_revised_just_now",
                "Unknown",
                "Unknown",
                "Unknown",
                "Data"
            ]
            .into_iter()
            .map(|f| f.to_string())
            .collect();
            new_row.extend(rows.clone());
            gotten_row_matches.push(new_row);

            println!("Just pushed a new row")
           
        }
    }
    if gotten_row_matches.len() > 0 {
        return Some(gotten_row_matches);
    }

    None
}
