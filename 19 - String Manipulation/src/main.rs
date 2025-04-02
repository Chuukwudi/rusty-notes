fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);
    
    // concatenate using format!
    // let description = format!("Title: Quick story\n{}", sentence);
    // println!("{}", description);
    
    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }
    
    // Split and collect into a vector
    // let words: Vec<&str> = sentence.split_whitespace().collect();
    // println!("{:?}", words);
    // OR
    // let words: Vec<String> = sentence.split_whitespace().map(|s| s.to_string()).collect();
    // println!("{:?}", words);
    // OR
    // let words: Vec<&str> = sentence.split(' ').collect::Vec<&str>();
    // println!("{:?}", words);

    // let reversed: String = sentence.chars().rev().collect:::<String>();
    // println!("{}", reversed);
}



// This code shows:

// Defining a string and using slicing to get the first five characters
// Several commented-out examples demonstrating different string operations:

// Using format! to create a new string
// Iterating over characters and matching vowels
// Splitting a string into words and collecting them into a vector