use std::collections::HashMap;

fn main() {
    // 创建一个新的 HashMap
    let mut scores = HashMap::new();

    // 向 HashMap 中插入值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 访问 HashMap 中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(&score) => println!("{} 的分数: {}", team_name, score),
        None => println!("未找到 {} 的分数", team_name),
    }

    // 遍历 HashMap 中的键值对
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新 HashMap 中的值
    scores.insert(String::from("Blue"), 25);
    println!("更新后 Blue 的分数: {}", scores.get("Blue").unwrap());

    // 只有在键没有值的情况下插入值
    scores.entry(String::from("Green")).or_insert(30);
    println!("Green 的分数: {}", scores.get("Green").unwrap());

    // 根据旧值更新值
    let yellow_score = scores.entry(String::from("Yellow")).or_insert(50);
    *yellow_score += 10;
    println!("更新后 Yellow 的分数: {}", scores.get("Yellow").unwrap());
}