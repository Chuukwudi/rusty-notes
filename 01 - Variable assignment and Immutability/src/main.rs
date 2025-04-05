fn main(){
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
}