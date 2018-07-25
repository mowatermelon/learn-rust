extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    loop {
        println!("\n---------------------Guess the number!\n");

        let secret_number = rand::thread_rng().gen_range(1, 101);
        loop {
            println!("Please input your guess:");

            let mut guess = String::new();

            io::stdin().read_line(&mut guess)
                .expect("failed to read line");

            let guess: u32 = guess.trim().parse()
                .expect("Please type a number!");

            match guess.cmp(&secret_number) {
                Ordering::Less    => println!("The number you guessed is too small!\n"),
                Ordering::Greater => println!("The number you guessed is too big!\n"),
                Ordering::Equal   => {
                    println!("You win!\n");
                    break;
                }
            }
        }
        println!("---------------------Yes,the secret number is: {}\n", secret_number);
    }
}
