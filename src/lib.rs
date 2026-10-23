use std::io;

fn get_random_word() -> String {
    let response = reqwest::blocking::get(
        "https://random-word-api.herokuapp.com/word"
    )
    .expect("Failed to get word")
    .error_for_status()
    .expect("Request failed");

    let words: Vec<String> = response
        .json()
        .expect("Failed to parse JSON");

    words.into_iter()
        .next()
        .expect("API returned no words")
}

struct Hangman {
    word: String,
    guessed_letters: Vec<char>,
    attempts: i32,
    max_attempts: i32
}

impl Hangman {
    fn new() -> Hangman {
        let word = get_random_word().trim().to_lowercase();
        Hangman { word, guessed_letters: Vec::new(), attempts: 0, max_attempts: 6 }
    }

    fn display_word(&self) {
        for letter in self.word.chars() {
            if self.guessed_letters.contains(&letter) {
                print!("{letter} ");
            } else {
                print!("_ ");
            }
        }
    }

    fn guess_letter(&mut self, letter: char) -> bool {
        if self.guessed_letters.contains(&letter) {
            println!("You already guessed that letter!");
            return false;
        }

        self.guessed_letters.push(letter);

        if self.word.contains(letter) {
            println!("Correct! Try another letter.")
        } else {
            self.attempts += 1;
            println!("Wrong! Attempts: {}/{}", self.attempts, self.max_attempts)
        }
        true
    }

    fn is_won(&self) -> bool {
        self.word.chars()
            .all(|c| self.guessed_letters.contains(&c))
    }

    fn is_over(&self) -> bool {
        self.is_won() || self.attempts >= self.max_attempts
    }
}

pub fn game_logic() {
    let mut game = Hangman::new();
    let mut guess = String::new();

    while !game.is_over() {
        game.display_word();

        println!("\tGuessed letters: {:?}", game.guessed_letters);
        println!("\nGuess a letter:");

        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let input = guess.trim().to_lowercase();
        let mut chars = input.chars();

        let Some(letter) = chars.next()
            .filter(|c| c.is_alphabetic())
            .filter(|_| chars.next().is_none())
        else {
            println!("Please enter a single letter!");
            continue;
        };

        game.guess_letter(letter);
    }

    if game.is_won() {
        println!("You won! The word was: {}", game.word);
    } else {
        println!("Game over! The word was: {}", game.word);
    }

}
