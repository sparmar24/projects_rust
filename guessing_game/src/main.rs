use rand::Rng; //random number gengerator
use std::cmp::Ordering;
use std::io; //input/output library

fn main() {  //main function
    println!("Guess the number!");   //print the statement
    
    let secret_number = rand::thread_rng().gen_range(1..=100); //generate a random number between
    //1-100

    // println!("The secret number is: {secret_number}");  //print statement
    
    loop {
        println!("Please input your guess."); //print statement
        
        let mut guess = String::new();  //define mutable variable
        io::stdin()  //
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {  //convert string to u32
            Ok(num) => num, //ok if guessed a number
            Err(_) => continue, //ignore if non-number
        };
        println!("You guessed: {guess}"); //print statement

        match guess.cmp(&secret_number) { //comparison of guess and secret number
            Ordering::Less => print!("Too small!"),
            Ordering::Greater => print!("Too big!"),
            Ordering::Equal => {
                print!("You win!");
                break;
            }
        }
    }
} 
