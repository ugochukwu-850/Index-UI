use std::collections::{HashMap, HashSet};

use calamine::{DataType, Range};
use rust_xlsxwriter::{Format, Worksheet};

use super::models::IndexError;

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
    let mut titles: Vec<String> = vec![
        "File Last Revised Date",
        "File Number",
        "Serial Number",
        "File Number - Serial Number",
        "Filename",
        "Data",
    ]
    .into_iter()
    .map(|f| f.to_string())
    .collect();

    for cell in excel.rows().next()? {

        // if update the titles row
        if cell.is_empty() {
            return None;
        }
        titles.push(cell.to_string());

    }
    if titles.len() < 6 {
        return None
    }
    return Some(titles);

}

/// Creates N amount of row instances each time a query Data is found
/// Returns a matrix of rows for each query instance being found <br>
/// Examples
/// ```rust
/// let example_response = vec!["name_of_query_found_in_row", "remaining row data"];
/// // println!("This would be an example of a response");
/// ```
pub fn filter_rows(
    row: &[DataType],
    query: &HashSet<String>,
    file_name: String,
) -> Option<Vec<Vec<String>>> {
    // if the row has any of the query then create a row instance with the row
    let mut check = HashSet::new();
    let rows: Vec<String> = row
        .iter()
        .map(|d| {
            let d = d.to_string();
            let r = cleanText(&d);
            check.insert(r.to_owned());
            r
        })
        .collect();
    let mut gotten_row_matches = Vec::new();

    for data in query {
        // println!("Data searching for {data}");
        if check.contains(data) {
            let mut new_row: Vec<String> = vec![
                "12/12/2023  10:05:00 PM",
                "Unknown",
                "Unknown",
                "Unknown",
                &file_name,
                data,
            ]
            .into_iter()
            .map(|f| f.to_string())
            .collect();
            new_row.extend(rows.clone());
            gotten_row_matches.push(new_row);

            // println!("Just pushed a new row")
        }
    }
    if gotten_row_matches.len() > 0 {
        return Some(gotten_row_matches);
    }

    None
}

/// Manages the titles index map and guarantees No duplicate in title row <br>
/// Takes `titlesmap: HashMap<String, usize>, title_index: usize, worksheet_handle: WorkSheet, titles: Vec<String>`;
pub fn merge_titles(
    titles: Vec<String>,
    title_map: &mut HashMap<String, usize>,
    cursor: &mut Worksheet,
    format: Option<Format>,
) -> Result<Vec<usize>, IndexError> {
    let mut titles_index = Vec::new();
    for title in titles {
        if !title_map.contains_key(&cleanText(&title.to_string())) {
            let title_index = title_map.len();
            // write cell data to the row
            if let Some(ref f) = format {
                cursor.write_with_format(0, title_index as u16, title.clone(), &f)?;
            } else {
                cursor.write(0, title_index as u16, title.clone())?;
            }
            titles_index.push(title_index);
            title_map.insert(cleanText(&title.to_string()), title_index);
        } else {
            titles_index.push(title_map.get(&cleanText(&title)).unwrap().to_owned());
        }
    }

    Ok(titles_index)
}

#[allow(unused)]
pub fn is_query(queries: &HashSet<String>, cell: String) -> bool {
    queries.contains(&cell)
}

pub fn file_index_gen(fileindex_map: &mut HashMap<String, usize>, filename: &String) -> usize {
    // if file in map return its index else return maps len
    if let Some(ind) = fileindex_map.get(filename) {
        ind.to_owned()
    } else {
        let ind = fileindex_map.len() + 1;
        fileindex_map.insert(filename.to_owned(), ind);
        ind
    }
}
