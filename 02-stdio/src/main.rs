use std::{io, cmp::Ordering};
use rand::Rng;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..10);


    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("You're failed luke");

        let guess: u32 = guess.trim().parse().expect("Please type a number");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess small"),
            Ordering::Greater => println!("Guess Big"),
            Ordering::Equal => {
                println!("you won, your guess is {} & secret is {}", guess, secret_number);
                break;
            },
        }
    }

}
