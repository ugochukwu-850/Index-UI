use std::collections::{HashMap, HashSet};

use calamine::{DataType, Range};
use chrono::format::StrftimeItems;
use rocket::{
    form::{self, Form},
    fs::FileName,
    serde::json::Json,
};
use rust_xlsxwriter::{Color, Format, Worksheet};

use super::models::{IndexError, JsonQuery};

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
        return None;
    }
    return Some(titles);
}

/// takes a range of cells and checks that they pass the validity test
/// If they pass it returns the titles
pub fn validity_1(
    excel: &Range<DataType>,
    query: HashMap<String, HashSet<String>>,
) -> Option<(Vec<String>, HashMap<usize, HashSet<String>>)> {
    // No empty first row
    let mut titles: Vec<String> = vec![
        "File Last Revised Date",
        "File Number",
        "Serial Number",
        "File Number - Serial Number",
        "Filename",
        "Title",
        "Data",
    ]
    .into_iter()
    .map(|f| f.to_string())
    .collect();

    let mut query_indexes: HashMap<usize, HashSet<String>> = HashMap::new();

    let first_row = excel.rows().next();

    if let Some(first_row) = first_row {
        for (index, cell) in first_row.iter().enumerate() {
            // if update the titles row
            if cell.is_empty() {
                return None;
            }
            titles.push(cell.to_string());

            if let Some((key, value)) = query.get_key_value(&cleanText(&cell.to_string())) {
                // save the index alongside the data
                query_indexes.insert(index, value.to_owned());
            }
        }
        if titles.len() < 6 {
            return None;
        }

        println!("P{:?}", query_indexes);
        return Some((titles, query_indexes));
    }

    None
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
    index: usize,
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
                &get_time(),
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
            new_row.push(index.to_string());
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
    query_type: &JsonQuery,
) -> Result<Vec<usize>, IndexError> {
    let mut titles_index = Vec::new();
    for (index, title) in titles.into_iter().enumerate() {
        if !title_map.contains_key(&cleanText(&title.to_string())) {
            let format = Some(gen_format(query_type, index, 0));

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

pub fn get_time() -> String {
    let now = chrono::Utc::now().naive_local();
    let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
    now.format_with_items(fmt).to_string()
}

///
/// 
pub fn get_file_trail(filename: Option<&FileName>) -> String {
    // check if the filename is safe
    match filename {
        Some(filename) => {
            let x_f = filename.dangerous_unsafe_unsanitized_raw();
            let extension = x_f.split(".").last().unwrap().as_str();

            format!("{}.{}", filename.as_str().unwrap(), extension)
        }
        None => String::from("Dangerous Or Missing filename.xlsx"),
    }
}

pub fn gen_format(search_type: &JsonQuery, cell_index: usize, row_index: usize) -> Format {
    let mut format = Format::new();

    match search_type {
        JsonQuery::TitleData(_) => format,
        JsonQuery::OnlyData(_) => {
            // if this is the first row

            if row_index == 0 {
                if cell_index == 5 {
                    format = format.set_background_color(Color::Cyan)
                }
                return format.set_bold().set_font_size(12);
            }
            if cell_index == 5 {
                return format
                    .set_bold()
                    .set_background_color(Color::Pink)
                    .set_font_size(12);
            }
            format
        }
    }
}

// fn to get the name of a file with its extension
// also figure out if the time is the time the file was processed or the time the reult was created
// or the files last modified date as a file

/// Creates N amount of row instances each time a query Data is found
/// Returns a matrix of rows for each query instance being found <br>
/// Examples
/// ```rust
/// let example_response = vec!["name_of_query_found_in_row", "remaining row data"];
/// // println!("This would be an example of a response");
/// ```
pub fn filter_rows_1(
    row: &[DataType],
    query_titles_index: &HashMap<usize, HashSet<String>>,
    workbook: &Range<DataType>,
    file_name: String,
    index: usize,
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

    for (title_index, query_data) in query_titles_index {
        println!("{:?}", query_data);
        if let Some(cell) = workbook.get((index, *title_index)) {
            if let Some(h) = query_data.get(&cleanText(&cell.to_string())) {
                let mut new_row: Vec<String> = vec![
                    &get_time(),
                    "Unknown",
                    "Unknown",
                    "Unknown",
                    &file_name,
                    &workbook.get((0, *title_index)).unwrap().to_string(),
                    &cell.to_string(),
                ]
                .into_iter()
                .map(|f| f.to_string())
                .collect();
                new_row.extend(rows.clone());
                new_row.push(index.to_string());
                gotten_row_matches.push(new_row);
            }
        }
    }
    if gotten_row_matches.len() > 0 {
        return Some(gotten_row_matches);
    }

    None
}
