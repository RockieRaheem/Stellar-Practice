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
}
