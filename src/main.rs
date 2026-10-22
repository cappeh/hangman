use std::io;

fn main() {
    let word = hangman::get_random_word().trim().to_lowercase();
    println!("{word}");
    let mut guess = String::new();
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut attempts = 0;
    let max_attempts = 6;

    while attempts < max_attempts {
        for letter in word.chars() {
            if guessed_letters.contains(&letter) {
                print!("{letter} ");
            } else {
                print!("_ ");
            }
        }

        println!("\nGuess a letter:");

        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let input = guess.trim().to_lowercase();

        if input.len() != 1 || !input.chars().all(|c| c.is_alphabetic()) {
            println!("Please enter a single letter!");
            continue;
        }

        let letter = input.chars().next().unwrap();

        if guessed_letters.contains(&letter) {
            println!("You already guessed that letter!");
            continue;
        }

        guessed_letters.push(letter);

        if word.contains(letter) {
            if word.chars().all(|c| guessed_letters.contains(&c)) {
                println!("You won! The word was: {word}");
                break;
            }
            println!("Correct! Try another letter.");
            continue;
        } else {
            attempts += 1;
            println!("Wrong! Attempts: {attempts}/{max_attempts}");

            if attempts == max_attempts {
                println!("Game over! The word was: {word}")
            }
        }
    }
}
