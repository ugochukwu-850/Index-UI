use rust_xlsxwriter::Workbook;

pub fn new_example() -> Vec<u8> {
    let mut workbook_handle = Workbook::new();

    let worksheet_handle = workbook_handle.add_worksheet();
    workbook_handle.save_to_buffer().unwrap().to_vec()
}