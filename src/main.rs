use core::num;
use rand::{thread_rng, Rng};
use std::io;

fn create_random_number() -> u8 {
    thread_rng().gen_range(1..=100)
}

fn is_first_value_greater_then_second(x: &u8, y: &u8) -> bool {
    x > y
}

fn get_user_number() -> Result<u8, num::ParseIntError> {
    let mut guess_attempt = String::new();
    io::stdin()
        .read_line(&mut guess_attempt)
        .expect("Unable to read guess_attempt");

    let result: u8 = guess_attempt.trim().parse()?;
    Ok(result)
}

fn handle_guess_correction(guess_attempt: &u8, guessed: &u8) {
    let is_greater = is_first_value_greater_then_second(&guess_attempt, &guessed);

    match is_greater {
        true => println!("Noo, lower expecations!"),
        false => println!("It's more!"),
    }
}

fn ask_user_for_guess(guessed: &u8) -> bool {
    let guess_attempt = get_user_number().expect("Number parsing error");

    match &guess_attempt {
        _ if &guess_attempt == guessed => {
            println!("Exactly!");
            true
        }
        _ => {
            handle_guess_correction(&guess_attempt, &guessed);
            false
        }
    }
}

fn main() {
    let value = create_random_number();

    println!("Guess the name from 1 to 100");
    while ask_user_for_guess(&value) != true {}
}
