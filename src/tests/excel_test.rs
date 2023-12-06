use std::collections::HashSet;

#[test]
fn test_index_hashzet() {
    let mut titles = HashSet::new();
    titles.insert("title1");
    titles.insert("titles2");
    titles.insert("title1");

    let mut y = vec![vec![0, 12]; 12];
    y[0][0] = 1;
    
    println!("{:?}", titles);
}