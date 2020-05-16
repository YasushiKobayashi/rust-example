extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 3);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        let res = guess_secret_number(guess, secret_number);
        if res {
            break;
        }
    }
}

fn guess_secret_number(guess: u32, secret_number: u32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }
    return false;
}

#[test]
fn guess_secret_number_same() {
    let res = guess_secret_number(1, 1);
    assert_eq!(res, true);
}

#[test]
fn guess_secret_number_not_same() {
    let large = guess_secret_number(1, 2);
    assert_eq!(large, false);

    let small = guess_secret_number(2, 1);
    assert_eq!(small, false);
}
