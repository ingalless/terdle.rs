use colored::*;
use rand::seq::IteratorRandom;

fn guess(target_word: &String) -> (bool, String) {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim().len() != 5 {
                println!("Word must be 5 letters long");
                return guess(&target_word);
            }

            return (input.trim() == target_word, input.trim().to_string());
        }
        Err(error) => panic!("error: {error}"),
    }
}

fn print_word(target_word: String, guess: String) {
    for i in 0..5 {
        let c = guess.chars().nth(i).unwrap().to_string();
        if target_word.chars().nth(i).unwrap().to_string() == c {
            print!("{}", c.green());
        } else if target_word.contains(&c) {
            print!("{}", c.yellow());
        } else {
            print!("{}", c);
        }
    }
    println!();
}

fn get_target_word() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://gist.githubusercontent.com/dracos/dd0668f281e685bad51479e5acaadb93/raw/6bfa15d263d6d5b63840a8e5b64e04b382fdb079/valid-wordle-words.txt")?
        .text()?;
    let lines = resp.lines().map(|c| c);
    let word = lines.choose(&mut rand::thread_rng()).expect("No lines");
    Ok(word.to_string())
}

fn print_guess_grid(target_word: String, guesses: Vec<String>) {
    print!("{}[2J", 27 as char);
    for i in 0..5 {
        match guesses.get(i) {
            Some(guess) => print_word(target_word.clone(), guess.to_string()),
            None => println!("-----"),
        }
    }
    println!("Enter your guess...");
}

fn main() {
    println!("Try to guess the word in 5 guesses...");
    let target_word = get_target_word().expect("Failed to fetch target word.");
    let max_guesses = 5;
    let mut won = false;
    let mut guesses = Vec::new();
    for i in 1..=max_guesses {
        print_guess_grid(target_word.clone(), guesses.clone());

        let (correct, guess) = guess(&target_word.to_string());
        guesses.push(guess);
        if correct {
            won = true;
            break;
        } else {
            println!("Not quite... ({} guesses remaining)", max_guesses - i);
        }
    }

    if won {
        print_guess_grid(target_word.clone(), guesses.clone());
        println!("You got it! Nice one.");
    } else {
        print_guess_grid(target_word.clone(), guesses.clone());
        println!("The word was '{}'", target_word);
    }
}
