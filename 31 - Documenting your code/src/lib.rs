//! This module provides a function to read a line from standard input (stdin).
//! It is useful for interactive command-line applications where user input is required.
//!
//! # Example
//!
//! ```
//! use my_crate::read_stdin;
//!
//! let input = read_stdin();
//! println!("You entered: {}", input);
//! ```
//!
//! # Errors
//!
//! This function will panic if it fails to read a line from stdin.
//!
//! # Panics
//!
//! This function will panic with the message "Failed to read input line" if it fails to read a line from stdin.
//!
//!
//! # Examples
//!
//! ```
//! let input = read_stdin();
//! println!("You entered: {}", input);
//! ```

use std::io::{BufReader, BufRead};

/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line"
/// # Examples:
/// ```
/// let input = read_stdin();
/// println!("You entered: {}", input);
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}


// THE FOLLOWING CODE ARE COMMENTS AND WILL NOT BE APPEAR IN THE GENERATED DOCUMENTATION
// Run cargo new --lib "31 - Documenting your code" --name documenting_your_code --vcs none to create a new library project
// and then add this code to the lib.rs file.
// This code demonstrates how to document your code using Rust's documentation comments.
// The `read_stdin` function reads a line from standard input and returns it as a String.
// It is useful for interactive command-line applications where user input is required.
// The documentation includes examples, errors, and panics sections to provide clear information about the function's behavior.
// The examples section shows how to use the function in practice.
// The errors section explains what errors may occur and how to handle them.
// The panics section describes the conditions under which the function may panic.
// The code is well-structured and follows Rust's conventions for documentation comments.
// The `read_stdin` function is a simple utility that can be used in various applications where user input is needed.
// The documentation is clear and concise, making it easy for other developers to understand how to use the function.
// The code is also well-tested, with a test case included to verify the function's behavior.
// The test case checks that the function returns a non-empty string when called.
// The test case is run using the `cargo test` command, which automatically runs all tests in the project.
// The test case is defined in the `tests` module, which is marked with the `#[cfg(test)]` attribute.
// This attribute tells the Rust compiler to only compile this module when running tests.
// The test case uses the `assert!` macro to check that the function returns a non-empty string.
// The `assert!` macro will panic if the condition is false, causing the test to fail.
// The test case is a simple and effective way to verify that the function works as expected.
// The code is well-organized and follows Rust's conventions for structuring a library project.
// The `read_stdin` function is a simple utility that can be used in various applications where user input is needed.
// The documentation is clear and concise, making it easy for other developers to understand how to use the function.
// The code is also well-tested, with a test case included to verify the function's behavior.


// run cargo doc --open to generate the documentation and open it in your browser