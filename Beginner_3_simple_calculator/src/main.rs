// Overview of the Code
// The program is a simple calculator that:
// - Repeatedly displays a menu for user input.
// - Lets the user choose an operation (addition, subtraction, multiplication, division).
// - Takes two numeric inputs for the operation.
// - Outputs the result or an error message (e.g., division by zero).
// - Allows the user to exit by selecting the "Exit" option.


use std::io;    // This imports the input/output module from Rust's standard library.
                // It allows reading input from the user via io::stdin() and writing outputs using println!.

fn display_menu() { //Displays the menu options to the user.
    println!("This is a simple calculator build on Rust!");
    println!("Please select the operation");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Exit");
}

fn add(a:f64,b:f64)->f64{
    a+b
}

fn subtract(a:f64,b:f64)->f64{
    a-b
}

fn multiply(a:f64,b:f64)->f64{
    a*b
}

fn divide(a:f64,b:f64)->Result<f64, &'static str>{
    if b==0f64 {
        Err("division by zero is not allowed")
    }
    else{
        Ok(a/b)
    }
}


fn get_input(prompt: &str) -> f64{ //Repeatedly prompts the user until valid numeric input is provided
    loop {
        println!("{} ", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>(){ //.trim() removes leading and trailing whitespace from user input.
            Ok(num)=>return num,
            Err(_)=>println!("Please Enter a valid input.")
        }
    }
}


fn main() {
    loop {
        display_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let num1 = get_input("Enter the First Number");
                let num2 = get_input("Enter the Second Number");

                println!("Result of Addition: {} + {} = {}", num1, num2, add(num1, num2));
            }

            "2" => {
                let num1 = get_input("Enter the First Number");
                let num2 = get_input("Enter the Second Number");

                println!("Result of Subtraction: {} - {} = {}", num1, num2, subtract(num1, num2));
            }

            "3" => {
                let num1 = get_input("Enter the First Number");
                let num2 = get_input("Enter the Second Number");

                println!("Result of Multiplication: {} x {} = {}", num1, num2, multiply(num1, num2));
            }

            "4" => {
                let num1 = get_input("Enter the First Number");
                let num2 = get_input("Enter the Second Number");
                match divide(num1, num2) {
                    Ok(result) => println!("Result of Division: {} / {} = {}", num1, num2, result),
                    Err(e) => println!("Error: {}", e),
                }
            }

            "5" => {
                println!("Exiting the Calculator, Goodbye");
                break;
            }
            _ => {
                println!("Enter a valid Option");
            }
        }
    }
}
