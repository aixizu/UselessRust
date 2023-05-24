use std::io::{self, Write};

fn is_palindrome(string: &str) -> bool
{
    // Create a new string to store the reversed version
    // of the input string
    let mut reversed: String = String::new();

    // Iterate over the characters in the input string in reverse order
    for char in string.chars().rev()
    {
        // Append each character to the reversed string
        reversed.push(char);
    }

    // Compare the input string and reversed string
    string == reversed
}

fn main()
{
    // Prompt the user to enter a string and flush the line buffer 
    // so it shows immediately!
    print!("Enter a string: ");
    io::stdout().flush().unwrap();

    // Store the user's input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");

    // Trim the user's input
    let palindrome: &str = input.trim();

    // Check if the string is a palindrome
    if is_palindrome(palindrome) 
    {
        println!("{} is a palindrome", palindrome);
    } else {
        println!("{} is not a palindrome", palindrome);
    }
}