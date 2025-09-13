use std::collections::BTreeMap;
mod balances;
fn main () {
    println!("Blockchain module loaded successfully.");

    let mut map = BTreeMap::new();
    map.insert("Alice", 100);
    assert_eq!(map.get(&"Alice"), Some(&100));
    assert_eq!(map.get(&"Bob"), None);

    let x = map.get("assssss");
     
}