fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    // This will not compile!
    result.expect("Oops! something went wrong").to_string()
}

fn main() {
    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}", chunk);
}


// This program demonstrates:

// A function that splits a string by a delimiter and returns a specific field
// Using Vec<&str> to collect parts of a split string
// Accessing a specific index with .get() which returns an Option<&T>
// Using .expect() to handle the case where the index is out of bounds
// Converting a string slice (&str) to an owned String with .to_string()

// When run, this code will split "hello,world" by the comma delimiter and return the field at index 1, which is "world". It then prints "Split string: world".
// The comment about not compiling is incorrect - this code should actually compile and run fine, as long as the requested field index exists in the split string.