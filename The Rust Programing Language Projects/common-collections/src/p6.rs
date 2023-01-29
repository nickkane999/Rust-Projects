use std::collections::HashMap;

pub fn run() {
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut map = HashMap::new();
    map.insert(blue, 10);
    map.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = map.get(&team_name);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 20);

    scores.entry("Blue").or_insert(30);
    scores.entry("Red").or_insert(50);    
}