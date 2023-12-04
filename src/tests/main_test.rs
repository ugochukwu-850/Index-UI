use crate::menu::knubs::cleanText;

#[test]
fn testclean_text() {
    let text = String::from("text\n\n\r\r\t\t  now go");
    let res = cleanText(&text);
    println!("{res:?}");
    assert_eq!(res.contains("\r"), false)
}