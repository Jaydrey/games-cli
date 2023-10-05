use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number");

    let _number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();
        println!("{}", "Please input a number".blue());
        // prompt user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert string to unsigned integer
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // compare the greaterness of numbers
        match guess.cmp(&_number) {
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You Win".green());
                break;
            },
            Ordering::Less => println!("{}", "Too small".red()),
        }
    }
}
