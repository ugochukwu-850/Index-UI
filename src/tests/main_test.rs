use std::collections::{HashSet, HashMap};

use crate::menu::knubs::{cleanText, get_bacth_index_from_proc_id, generate_index};

#[test]
fn testclean_text() {
    let text = String::from("text\n\n\r\r\t\t  now go");
    let res = cleanText(&text);
    println!("{res:?}");
    assert_eq!(res.contains("\r"), false)
}


#[test]
fn test_get_index(){
    let proc_id = "duheih@1".to_string();
    let id = get_bacth_index_from_proc_id(&proc_id);
    assert_eq!(id, 1);
}

#[test]
fn test_gen_index() {
    let mut indexes = vec![];
    for batch in 0..10 {
        for file_index in 0..10 {
            let file_batch_index = generate_index(batch, file_index);
            indexes.push(file_batch_index);
        } 
    }
    println!("{:?}", indexes);
    let mut as_set = HashSet::new();
    for y in indexes.to_owned() {
        as_set.insert(y);
    }
    assert_eq!(indexes.len(), as_set.len());
}

#[test]
fn maps_test() {
    let (mut a_s, mut a_b) = (Vec::new(), Vec::new());

    a_s.extend(["1", "2", "3", "6", "iter"]);
    a_b.extend(["iter","not", "1"]);
    let v = vec![vec!["name", "brother", "her"], vec!["yourname", "1"]];

    let r_a = a_b.extend(a_s);

    println!("{a_b:?}");

}

#[test]
fn gen_titles() {
    // loop through all the titles and create a map only if the title is unique

    let titles = vec!["titles1", "titles1", "titles2 ", "titles 2", "titles3"];

    // create a map for the title and its index and also create a new vector a unique titles
    let mut clean_title_index_map = HashMap::new(); 
    let mut titles_vec_set = Vec::new();

    for title in titles {
        if !clean_title_index_map.contains_key(&cleanText(&title.to_string())) {
            titles_vec_set.push(title);
            clean_title_index_map.insert(cleanText(&title.to_string()), titles_vec_set.len() - 1);
        }
    }

    println!("Clean Map: {:?} \n Vec Set: {:?}", clean_title_index_map, titles_vec_set);
}

#[test]
fn test_main_x() {
    // create a hashmap to store the filenames index as cleaned index
    let _file_index_map: HashMap<&str, usize> = HashMap::new();

    // the cleaned titles map is now gotten from other function 
    let _clean_title_index_map: HashMap<String, usize> = HashMap::new();
    
    // with the vec set titles prepend the abitary infos

    //  loop through every row as gotten from merge

    //  create an abitary number of cells as long as titles_Len + default cols

    // Add first five data by using this format
    // - filename : Given filename from row gen
    // - fileindex : map_index for cleaned filename "function should maintain mapping"
    // - index : enumerates index
    // - last revised: As given from row
    // - file index + index : DO the math
    // - Data: as given

    // Now handle each column as a different enum variant as they would hold diff info
    // If the variant is among the above just threat accordingly 

    // for each cell data Variant((cleanedText("Title"), "celldata"))
    // - use its title key to get the index and set it on the virtual row earlier created
    // - Once done for all : Push to virtual body matrix

    // Once that has been done :
    // - Push the titles and the body to main matrix
    // - Give this matrix to the function that creates the excel file
    
    // creating a matrix of enums instead so that the compiling function can know before hand
    // what cells to format with colors without attempting  to check for all
    
    // example :
    // match cellO(celldata) => CellQuery{ format with this color}, CellSpecialblue => {format with designed color}
    
}