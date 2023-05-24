use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    // generate secret number >:))
    let secret: i32 = rand::thread_rng().gen_range(1..=100);

    // keep track of user's attempts to guess the secret number
    let mut attempts: i32 = 0;

    loop 
    {
        // prompt the user and save their guess!
        print!("Enter your guess (1-100): ");
        io::stdout().flush().unwrap();

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read input");

        // parse the input and add 1 guess O:
        let guess: i32 = guess.trim().parse().expect("invalid input!");
        attempts += 1;

        // check if the user guessed correctly!
        match guess.cmp(&secret) 
        {
            Ordering::Less => println!("too low!"),
            Ordering::Greater => println!("too high!"),
            Ordering::Equal => 
            {
                // yippeee!
                println!("nice!!! you got the number!\nyou guessed it in {} attempts!!", attempts);
                break;
            }
        }

        // check if the user's guess is within 10 numbers of the secret number
        if (secret - guess).abs() <= 10
        {
            println!("but you're close! ");
        }

        // check if the user's guess is within 3 numbers of the secret number
        if (secret - guess).abs() <= 3 {
            println!("like really close!!");
        }
    }
}
