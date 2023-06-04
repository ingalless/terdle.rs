fn guess(target_word: &String) -> bool {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            if input.trim().len() != 5 {
                println!("Word must be 5 letters long");
                guess(&target_word);
            }
            return input.trim() == target_word;
        }
        Err(error) => panic!("error: {error}"),
    }
}

fn get_target_word() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://random-word-api.herokuapp.com/word?length=5")?
        .json::<Vec<String>>()?;
    println!("{:#?}", resp);
    let word = resp[0].clone();
    Ok(word)
}

fn main() {
    println!("Hello, world!");
    let word = get_target_word().expect("Failed to fetch target word");
    let max_guesses = 5;
    let mut won = false;
    for i in 1..=max_guesses {
        if guess(&word.to_string()) {
            won = true;
            break;
        } else {
            println!("Not quite... ({} guesses remaining)", max_guesses - i);
        }
    }

    if won {
        println!("You got it! Nice one.");
    } else {
        println!("The word was '{}'", word);
    }
}
