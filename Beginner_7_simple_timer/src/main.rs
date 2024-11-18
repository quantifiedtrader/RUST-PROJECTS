use std::io;
use std::time::{Duration, Instant};
use std::thread::sleep;

/// Entry point of the program
fn main() {
    println!("Welcome to the Simple Timer!");

    // Get duration from the user
    let duration = get_timer_duration();

    // Start the countdown
    println!("\nStarting the timer for {} seconds...", duration);
    start_timer(duration);

    // Timer end notification
    println!("\nTime's up!");
}

/// Reads the timer duration from the user and validates it
fn get_timer_duration() -> u32 {
    loop {
        println!("Enter the timer duration (in seconds):");

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse input into a number
        match input.trim().parse::<u32>() {
            Ok(num) => {
                if num > 0 {
                    return num;
                } else {
                    println!("Please enter a positive number.");
                }
            }
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

/// Starts the countdown timer
fn start_timer(duration: u32) {
    let start_time = Instant::now();
    for i in (1..=duration).rev() {
        println!("\rTime remaining: {} seconds", i);
        sleep(Duration::from_secs(1));
    }

    // Calculate total elapsed time
    let elapsed = start_time.elapsed();
    println!(
        "\nTimer completed in {:.2} seconds.",
        elapsed.as_secs_f64()
    );
}