use rand::Rng; //To generate random numbers
use std::io;    // For handling user input
use std::cmp::Ordering; //For comparison between numbers


fn main(){
    println!("Welcome to the Guess the Number Game!");
    println!("I have chosen a number between 1 and 100. Can you guess it?");
    println!("You have 10 attempts. Good luck!");

    // Generate a Random Number between 1 and 100

    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut attempts = 10; // Set the attempt limit

    while attempts > 0 {
        //Prompt the user for the input
        println!("\nAttempts remaining: {}", attempts);
        println!("Please input your guess.");

        //Create a mutable string to store the user input
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // compare the guess with secret number

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // EXit Loop
            }
        }
        // Decrement the attempts
        attempts -= 1;
    }
    // If the loop exits, the player has run out of attempts
    println!("\nSorry, you've run out of attempts. The number was: {}", secret_number);
}
