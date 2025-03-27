fn main() {
    let message = "Name: Chukwudi, Weight: ";
    let weight = 190.0;
    
    let kilos = weight / 2.2;
    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);

    // In the above code, variables are immutable by default. This means that
    // they can not be changed. 
    // To give any variable the ability too be mutated, you append it with the 
    // `mut` keyword during the declaration.
    // For example:
    let mut message = "Name: Chukwudi, Weight: ";
    message = "Name: Chukwudi, Weight in kilos: ";
    let mut weight = 190.0;
    weight = 200.0;
    let kilos = weight / 2.2;
    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);



    let topic = "04 - Other conditional statements (if let)";
    let folder_name = &topic[5..]
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("(", "")
        .replace(")", "");
    println!("{}", folder_name);
    let command = format!("cargo new \"{}\" --name {} --vcs none", topic, folder_name);
    println!("{}", command);
}