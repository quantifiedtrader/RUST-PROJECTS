// Workflow
// 1. Input File Path
//      The program expects the file path as a command-line argument. It validates this argument and then processes the file.
 
// 2. File Processing
//      The program reads the file contents line by line and splits each line into words. These words are normalized by:
 
//  -   Removing punctuation (non-alphanumeric characters).
//  -   Converting them to lowercase to ensure case-insensitivity.
 
// 3. Word Counting
// Each normalized word is stored in a HashMap, where the key is the word, and the value is the count of occurrences.
 
// 4. User Interaction
// After processing, the program displays basic statistics:
 
//      - Total word count.
//      - Top 10 most frequent words.
//      - The program then enters an interactive menu allowing the user to:
 
//          - View the total word count.
//          - Check the frequency of a specific word.
//          - Display the top n frequent words.
//          - Exit the program.

// 5. Error Handling
//      The program checks for:
//       - Missing or invalid file paths.
//       - Empty or invalid user inputs.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
struct WordCounter {
    word_count : HashMap<String,usize>, //A HashMap where keys are words and values are their frequencies.
}

impl WordCounter
{   // initializes an empty WordCounter
    fn new()->Self{
        WordCounter{
            word_count : HashMap::new()
        }
    }

    // Read File: Reads the entire file into a string using fs::read_to_string.
    // Split Words: Splits each line into words using split_whitespace.
    // Normalize Words: Removes punctuation and converts words to lowercase.
    // Update Count: Updates the word frequency in the HashMap.

    fn process_file(&mut self,file_path:&str) -> Result<(),io::Error> {
        let content = fs::read_to_string(file_path)?; //Reads file content
        for line in content.lines() {
            let words = line
                .split_whitespace()
                .map(|word| word.trim_matches(|c: char| !c.is_numeric()).to_lowercase());
            for word in words {
                if !word.is_empty() {
                    *self.word_count.entry(word).or_insert(0) += 1;
                }
            }
        }
        Ok(())
    }
    // Calculates the total number of words by summing the values in the HashMap:
    fn get_total_word_count(&self) -> usize {
        self.word_count.values().sum()
}
    // Fetches the count for a specific word:

    fn get_word_frequency(&self, word: &str) -> usize {
        self.word_count
            .get(&word.to_lowercase())
            .copied()
            .unwrap_or(0)
    }

    //Converts the HashMap into a vector of (word, frequency) pairs.
    // Sorts the vector by frequency in descending order.
    // Returns the top n words
    fn get_top_frequent_words(&self, n: usize) -> Vec<(String, usize)> {
        let mut word_freq: Vec<(String, usize)> = self.word_count.clone().into_iter().collect();
        word_freq.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by descending frequency
        word_freq.into_iter().take(n).collect()
    }


    // Displays overall word statistics:

    fn display_statistics(&self) {
        println!("Total Words: {}", self.get_total_word_count());
        println!("Top 10 Frequent Words:");
        for (word, count) in self.get_top_frequent_words(10) {
            println!("{:<15} {}", word, count);
        }
    }

}


// Input File Path: Read the file path from the command-line argument.
// File Validation: Check if the file exists and is readable.
// Process File: Read, normalize, and count words in the file.
// Display Statistics: Show total word count and top frequent words.
// Interactive Menu: Provide user options to explore data further.
// Error Handling: Handle invalid inputs or missing files gracefully.

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = if args.len() < 2 {
        println!("Please enter the file path:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string()
    } else {
        args[1].clone()
    };

    let mut word_counter = WordCounter::new();

    match word_counter.process_file(&file_path) {
        Ok(_) => {
            word_counter.display_statistics();
            loop {
                println!("\nOptions:");
                println!("1. Get total word count");
                println!("2. Get frequency of a specific word");
                println!("3. Get top frequent words");
                println!("4. Exit");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");

                match input.trim() {
                    "1" => {
                        println!("Total Word Count: {}", word_counter.get_total_word_count());
                    }
                    "2" => {
                        println!("Enter the word:");
                        let mut word = String::new();
                        io::stdin().read_line(&mut word).expect("Failed to read word");
                        let word = word.trim();
                        println!(
                            "Frequency of '{}': {}",
                            word,
                            word_counter.get_word_frequency(word)
                        );
                    }
                    "3" => {
                        println!("How many top words to display?");
                        let mut num = String::new();
                        io::stdin().read_line(&mut num).expect("Failed to read number");
                        match num.trim().parse::<usize>() {
                            Ok(n) => {
                                println!("Top {} Words:", n);
                                for (word, count) in word_counter.get_top_frequent_words(n) {
                                    println!("{:<15} {}", word, count);
                                }
                            }
                            Err(_) => println!("Please enter a valid number."),
                        }
                    }
                    "4" => {
                        println!("Exiting the program. Goodbye!");
                        break;
                    }
                    _ => println!("Invalid choice. Please try again."),
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
