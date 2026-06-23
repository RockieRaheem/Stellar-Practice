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


}