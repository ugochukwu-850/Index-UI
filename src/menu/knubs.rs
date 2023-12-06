

pub fn cleanText(text: &String) -> String {
    let result: String = text
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    result
}


pub fn get_bacth_index_from_proc_id(proc_id: &String) -> usize {
    let id = proc_id.split_once("@").unwrap();
    id.1.to_string().parse::<usize>().unwrap()
}

pub fn generate_index(batch_index: usize, index: usize) -> String {
    format!("{}@{}", batch_index, index)
}