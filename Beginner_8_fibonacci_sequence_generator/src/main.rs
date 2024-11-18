use std::io;

fn main(){
    println!("Welcome to the Fibonacci Sequence Generator");

    // Get the number of terms to be printed fom the user

    let num_terms = get_number_of_terms();

    //Generate Fibonacci Sequence iteratively

    println!("\nIterative Method:");
    let sequence = generate_fibonacci_iterative(num_terms);
    print_sequence(&sequence);


    println!("\nRecursive Method:");
    for i in 0..num_terms{
        println!("{}",generate_fibonacci_recursive(i));
    }

    println!("\n\nProgram completed successfully!");
}


//Read and Validate the user input for the number of terms

fn get_number_of_terms()->u32{
    loop{
        println!("Enter the number of terms (must be a positive integer):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<u32>(){
        Ok(num) if num > 0 => return num,
            _ => println!("Invalid input, Please enter a positive Integer")
        }
    }
}

//Generate the Fibonacci Sequence iteratively
fn generate_fibonacci_iterative(num_terms: u32) -> Vec<u32>{
    let mut sequence = Vec::new();
    let mut a = 1;
    let mut b = 1;

    for _ in 0..num_terms{
        sequence.push(a);
        let temp = a+b;
        a=b;
        b= temp;
    }
    sequence
}


fn generate_fibonacci_recursive(num_terms: u32) -> u32{
    match num_terms{
        0=>0,
        1=>1,
        _ => generate_fibonacci_recursive(num_terms-1)+generate_fibonacci_recursive(num_terms-2),
    }
}



// The recursive method is inefficient for large inputs due to repeated calculations.
// Add memoization to improve performance:

// fn generate_fibonacci_recursive_memoized(n: u32, memo: &mut Vec<u32>) -> u32 {
//     if n < memo.len() as u32 {
//         return memo[n as usize];
//     }
//     let result = generate_fibonacci_recursive_memoized(n - 1, memo)
//         + generate_fibonacci_recursive_memoized(n - 2, memo);
//     memo.push(result);
//     result
// }
// let mut memo = vec![0, 1];
// for i in 0..num_terms {
// print!("{} ", generate_fibonacci_recursive_memoized(i, &mut memo));
// }


//print the fibonacci sequence

fn print_sequence(sequence:&Vec<u32>){
    for num in sequence{
        println!("{}",num);
    }
    println!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterative() {
        assert_eq!(generate_fibonacci_iterative(5), vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_recursive() {
        assert_eq!(generate_fibonacci_recursive(5), 5);
        assert_eq!(generate_fibonacci_recursive(6), 8);
    }
}
