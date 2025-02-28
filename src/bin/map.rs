use std::collections::HashMap;

fn main() {
    // Creating a new HashMap
    let mut scores = HashMap::new();

    // Inserting values into the HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing values in the HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(&score) => println!("Score for {}: {}", team_name, score),
        None => println!("No score found for {}", team_name),
    }

    // Iterating over key-value pairs in the HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a value in the HashMap
    scores.insert(String::from("Blue"), 25);
    println!("Updated score for Blue: {}", scores.get("Blue").unwrap());

    // Only inserting a value if the key has no value
    scores.entry(String::from("Green")).or_insert(30);
    println!("Score for Green: {}", scores.get("Green").unwrap());

    // Updating a value based on the old value
    let yellow_score = scores.entry(String::from("Yellow")).or_insert(50);
    *yellow_score += 10;
    println!("Updated score for Yellow: {}", scores.get("Yellow").unwrap());
}