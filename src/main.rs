fn guess(target_word: &String) -> (bool, String) {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim().len() != 5 {
                println!("Word must be 5 letters long");
                guess(&target_word);
            }

            return (input.trim() == target_word, input.trim().to_string());
        }
        Err(error) => panic!("error: {error}"),
    }
}

fn get_target_word() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://random-word-api.herokuapp.com/word?length=5")?
        .json::<Vec<String>>()?;
    // println!("{:#?}", resp);
    let word = resp[0].clone();
    Ok(word)
}

fn print_guess_grid(guesses: Vec<String>) {
    for i in 0..5 {
        match guesses.get(i) {
            Some(guess) => println!("{}", guess),
            None => println!("-----"),
        }
    }
}

fn main() {
    println!("Try to guess the word in 5 guesses...");
    let word = get_target_word().expect("Failed to fetch target word.");
    let max_guesses = 5;
    let mut won = false;
    let mut guesses = Vec::new();
    for i in 1..=max_guesses {
        let (correct, guess) = guess(&word.to_string());
        guesses.push(guess);
        if correct {
            won = true;
            break;
        } else {
            println!("Not quite... ({} guesses remaining)", max_guesses - i);
        }
        print_guess_grid(guesses.clone());
    }

    if won {
        println!("You got it! Nice one.");
    } else {
        println!("The word was '{}'", word);
    }
}
