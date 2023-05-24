use std::io::{self, Write};

fn is_palindrome(string: &str) -> bool
{
    // create a new string to store the reversed version
    // of the input string
    let mut reversed: String = String::new();

    // iterate over the characters in the input string in reverse order
    for char in string.chars().rev()
    {
        // ppend each character to the reversed string
        reversed.push(char);
    }

    // compare the input string and reversed string
    string == reversed
}

fn main()
{
    // prompt the user to enter a string and flush the line buffer 
    // so it shows immediately!
    print!("Enter a string: ");
    io::stdout().flush().unwrap();

    // store the user's input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");

    // trim the user's input
    let palindrome: &str = input.trim();

    // check if the string is a palindrome
    if is_palindrome(palindrome) 
    {
        println!("{} is a palindrome", palindrome);
    } else {
        println!("{} is not a palindrome", palindrome);
    }
}