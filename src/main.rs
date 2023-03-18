// import play_wordle function from wordle.rs
mod wordle;
use wordle::play_wordle;

pub fn check_allowed_words(word: &str, word_list: Vec<String>) {
    // check if word is in allowed list of words
    if word_list.contains(&word.to_string()) {
    } else {
        // throw error that word is not in list
        panic!("Word provided not in list of allowed words");
    }
}

fn main() {
    // allow for list of input
    let input = ["ghost", "there", "wrote"];
    let mut total_tries = 0;

    // get list of potential words
    let word_list = vec!["ghost", "there", "wrote"];
    let word_list = word_list
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    // CHANGE TO BIG DATA SOURCE

    // for each one of the input, run through wordle function
    for word in input.iter() {
        // check if word is in allowed list of words
        check_allowed_words(word, word_list.clone());

        let guesses = play_wordle(word, word_list.clone());
        total_tries += guesses;
    }

    let len = input.len() as i32;
    println!("Average tries: {}", total_tries / len);
}
