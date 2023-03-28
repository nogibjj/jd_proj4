// model to make guess

// make first guess
// always the same guess
use rand::Rng;

pub fn make_first_guess() -> String {
    "salet".to_string()
}

// make best subsequent guess
// takes in letters and calcs max expectation of word or letters?
// finds word and returns guess

pub fn get_guess(word_list: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..word_list.len());
    word_list[random_number].to_string()
}
