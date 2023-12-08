use rust_xlsxwriter::Workbook;

pub fn new_example() -> Vec<u8> {
    let mut workbook_handle = Workbook::new();

    let worksheet_handle = workbook_handle.add_worksheet();
    workbook_handle.save_to_buffer().unwrap().to_vec()
}

pub fn new_excel_file(matrix: Vec<Vec<&str>>) -> Vec<u8> {
    let mut workbook_handle = Workbook::new();

    let worksheet_handle = workbook_handle.add_worksheet();

    let _ = worksheet_handle.write_row_matrix(0, 0, matrix);

    worksheet_handle.autofit();

    workbook_handle.save_to_buffer().unwrap().to_vec()
}

pub fn new_excel_file_t(matrix: Vec<Vec<String>>) -> Vec<u8> {
    let mut workbook_handle = Workbook::new();

    let worksheet_handle = workbook_handle.add_worksheet();

    let _ = worksheet_handle.write_row_matrix(0, 0, matrix);

    worksheet_handle.autofit();

    workbook_handle.save_to_buffer().unwrap().to_vec()
}

