use std::collections::HashSet;

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