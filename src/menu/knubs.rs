pub fn cleanText(text: &String) -> String {
    let matches = ["\r", "\n", "\t"];
    let mut res = String::new();
    for m in matches {
        res = text.replace(m, "");
    }
    res
}
