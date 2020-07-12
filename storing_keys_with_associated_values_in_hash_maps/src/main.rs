use std::collections::HashMap;

fn main() {
    // creating
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // all keys must have the same type
    // all values must have the same type

    // hash map takes the ownership of the values
    let team = String::from("Red");
    scores.insert(team, 100);
    // team is not valid after this

    // accessing values
    let team_name = String::from("Blue");

    // iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating
    scores.insert(String::from("Yellow"), 10); // overwrite
    scores.entry(String::from("Yellow")).or_insert(50); // insert if it doesnt exist
    scores.entry(String::from("Orange")).or_insert(60);
    println!("{:#?}", scores);
}
