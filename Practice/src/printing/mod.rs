// Standard library
use std::io;
use rand::Rng;

// Public module called printing
pub mod printing {
    use std::{cmp::Ordering, io};
    use rand::Rng;

    pub fn test(){
        println!("TEST");
    }

    pub fn get_input(){
        // Macro to print to screen
        println!("GUESS A NUMBER");

        // Variable to store input
        let mut guess = String::new();

        io::stdin()
            // Read the line, and assign the input to guess, as a reference
            .read_line(&mut guess)
            // Expect failure
            .expect("Failed to read");

        // Printing values requires the {}, placeholder to print the value
        println!("You guessed {}", guess);
    }

    pub fn guess(){
        // rand::thread_rng(): Function that gives us particular random number generator 
        // gen_range: Takes a range expression
        // start..=end, 1 to 100
        let guess: i32 = rand::thread_rng().gen_range(1..=100);
        println!("THE NUMBER IS {}", guess)
    }

    pub fn loop_guess_game(){
        println!("GUESS THE NUM");
        
        // Create a random num between 1 - 100
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("Please input your guess.");
            
            // Declare a string
            let mut guess = String::new();
    
            // Get the input, read the line, and handle error
            io::stdin()
                .read_line(&mut guess)
                .expect("FAILED TO READ");
    
            // Parse and format string
    
            // trim(): Will eliminate white space
            // parse(): Converts a string to another type, returns result type, Result is a enum with variants Ok and Err
            // let guess: u32 = guess.trim().parse().expect("Please type a number");
            
            // Using a match expression
            
            let guess: u32 = match guess.trim().parse() {
                // If we can turn string to number
                Ok(num) => num,

                // If we cant turn string to number
                // (_) is a catchall value
                Err(_) => continue,
            };
    
            match guess.cmp(&secret_number){
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                // Break loop if corrct guess
                Ordering::Equal => {
                    println!("YOU WIN!");
                    break;
                }
            }
        }
    }

}