
use std::io;

fn main() {

    //Prompts the user to enter their name
    println!("Please Enter your Name : ");

    // Creates a mutable variable to store the input
    // The name variable in this code is mutable (mut)
    // because we need to modify it by storing user input into it.
    // Further name is a String type because we’re dealing with user input,
    // which is naturally text data that can vary in length.

    let mut name = String::new();

    //Read User input and save it in the above declared mutable variable
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    //Trim any newline or extra whitespace from the input
    let name = name.trim();


    // When to Use &str vs. String
    //     &str: Use this when you want an efficient, read-only view of a string.
    //     Since trim() only removes whitespace at the edges, the result is logically
    //     the same data, just a smaller slice.
    //     String: Use String when you need ownership or the ability to modify the string
    //     (e.g., concatenation, mutation).

    //If input is empty, default to "WORLD"
    if name.is_empty() {
        println!("Hello World!");
    } else {
        println!("Hello {}", name);
    }
}


