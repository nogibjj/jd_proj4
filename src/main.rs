// import play_wordle function from wordle.rs
use wordle_solver::wordle::play_wordle;

fn main() {
    // allow for list of input
    let input = ["ghost", "there", "wrote"];
    let mut total_tries = 0;

    // for each one of the input, run through wordle function
    for word in input.iter() {
        let res = play_wordle(word);
        total_tries += res.1;
    }
    println!("Average tries: {}", total_tries / input.len());
}
