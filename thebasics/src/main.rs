use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1,101);
    println!("Welcome to the guessing game!\n\r\n");
    println!("Guess a number from 1 to 100 babyyy\n\r\n");
    println!("Secret Number: {}", secret_number);
    loop{
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Your guess is {}", guess);
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too small, try again".red()),
            Ordering::Equal=>{println!("{}","You win!".green()); break;},
            Ordering::Greater=>println!("{}","Too big, try again".red())
        }
    }
}
