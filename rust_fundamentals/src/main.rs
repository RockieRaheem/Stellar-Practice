use std::collections::HashMap;

fn main() {
    // Create a vector to hold the tech stack
    let mut tech_stack: Vec<&str> = Vec::new();

    tech_stack.push("Rust");
    tech_stack.push("JavaScript");
    tech_stack.push("Python");  

    println!("Tech Stack: {:?}", tech_stack);

    for stack in tech_stack {
      println!("The stack is: {}", stack);
    }

    // Create a vector to hold Top Scorers

    let mut top_scorers: Vec<String> = Vec::new();


    top_scorers.push("Mbappe".to_string());
    top_scorers.push("Messi".to_string());
    top_scorers.push("Halaand".to_string());

    for player in &top_scorers {
        println!("Top scorer is: {}", player);
    }

    // Create a phonebook hashmap

    let mut contacts: HashMap<&str, &str> = HashMap::new();

    contacts.insert("Alex", "0765432345");
    contacts.insert("Viola", "0768564334");

    match contacts.get("Alex") {
        Some(number) => println!("Alex's number is {}", number),
        None => println!("Not found in contacts."),
    }

}