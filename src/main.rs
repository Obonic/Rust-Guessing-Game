extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    println!("Welcome to the Guessing game! Guess a number between 1-100, you get 10 guesses to win!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("The Password is = {}", secret_number);
    let mut iteration = 0;
    let mut tries = 0;

    loop 
    {
        if iteration == 10
        {
            println!("You lose! The number was {}!", secret_number);
            break;
        }
            println!("Input Guess...");

            let mut guess = String::new();
            
            tries = tries + 1;

            io::stdin().read_line(&mut guess).expect("Failed!!!");

            println!("You guessed:{}", guess);

            match guess.trim().parse::<u32>()
            {
                Ok(parse_guess) => 
                {
                match parse_guess.cmp(&secret_number){
                Ordering::Less => println!("Too Small!!"),
                Ordering::Greater => println!("Too Large!!"),
                Ordering::Equal => 
                    {
                        println!("You WIN!!! It only took you {} tries!", tries);
                        break;
                    },
                } 
                iteration = iteration + 1;
            },
            Err(_) =>  println!("This not a number. :( ")
        } 
    }
}