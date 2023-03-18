// model to make guess

// make first guess
// always the same guess

pub fn make_first_guess() -> String {
    "salet".to_string()
}

// make best subsequent guess
// takes in letters and calcs max expectation of word or letters?
// finds word and returns guess

pub fn get_guess(word_list: Vec<String>) -> String {
    println!("word list: {:?}", word_list);
    "ghost".to_string()
}
