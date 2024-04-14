use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::{thread, time};


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut _guess_count: u32 = 0; 
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess} ");
        _guess_count += 1;
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win! You guessed {_guess_count} times in total.");
                let sleep_time = time::Duration::from_millis(5000);
                thread::sleep(sleep_time);
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
