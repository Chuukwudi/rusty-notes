struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

struct Point(i32, i32, i32);

fn main() {
    let username = String::from("johndoe");
    let email = String::from("john@example.com");
    let uri = String::from("https://example.com");
    let active = true;
    
    let user = User { username, email, uri, active };
    let my_point = Point(10, 20, 30);
    
    println!("points: {}", my_point.0);
}


// This code defines two structs:

// A User struct with named fields for username, email, URI, and active status
// A tuple struct Point with three i32 values

// The main function creates a User instance and a Point instance, then prints the first value of the Point tuple.