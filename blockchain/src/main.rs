use std::collections::BTreeMap;
mod balances;
fn main () {
    println!("Blockchain module loaded successfully.");

    let mut map = BTreeMap::new();
    map.insert("Alice", 100);
    assert_eq!(map.get(&"Alice"), Some(&100));
    assert_eq!(map.get(&"Bob"), None);

    let maybe_value: Option<&i32> = map.get("Alice");
    match maybe_value {
        Some(value) => println!("Found value: {}", value),
        None => println!("Value not found"),
    }
     
}