fn guess(target_word: String) -> bool {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input.trim() == target_word;
        }
        Err(error) => panic!("error: {error}"),
    }
}

fn main() {
    println!("Hello, world!");
    let word = "happy";
    let max_guesses = 5;
    for guesses in 1..=max_guesses {
        if guess(word.to_string()) {
            println!("Got it in {}! Nice one.", guesses);
            break;
        }
    }
}
