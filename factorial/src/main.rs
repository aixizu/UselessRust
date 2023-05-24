use std::io::{self, Write};

fn main()
{
    // Prompt the user and flush the line buffer
    // so we see it immediately!
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    // Store result
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");

    // Parse the input
    let number = input.trim().parse::<u32>().expect("Error parsing number");
    println!("You entered: {}", number);

    // Calculate the factorial of the number
    let mut factorial: u32 = 1;
    for i in 1..number + 1
    {
        factorial *= i;
    }

    println!("The Factorial of {} is: {}", number, factorial);
}
