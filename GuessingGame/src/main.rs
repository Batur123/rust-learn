use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut attempt_number: i8 = 5;

    let random_number = rand::thread_rng().gen_range(1..=10);
    println!("{}", random_number);

    loop {
        if attempt_number <= 0 {
            println!("You used all your attempts.");
            break;
        }

        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input is not a number");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => {
                println!("Too small!");
                attempt_number -= 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                attempt_number -= 1;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}