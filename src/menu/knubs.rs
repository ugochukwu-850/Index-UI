pub fn cleanText(text: &String) -> String {
    let result: String = text
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    result
}
