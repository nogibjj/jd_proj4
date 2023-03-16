// process wordle guess

// based on guess
// calc which letters are correct, misplaced, or incorrect

// make the colored result and return

use owo_colors::{
    colors::{Black, Green, Yellow, xterm::Gray},
    OwoColorize,
};
use std::fmt;


fn format_guess( d: impl fmt::Display, color:Str) -> String {
    if color == "green" {
        let res = format!("{}", d.fg::<Black>().bg::<Green>());
    } else if color == "yellow" {
        let res = format!("{}", d.fg::<Black>().bg::<Yellow>());
    } else if color == "grey" {
        let res = format!("{}", d.fg::<Black>().bg::<Gray>());
    }
    res
}

pub fn process_guess(guess:Str, true_word:Str) -> (i32) {
    // find which characters in guess are the same as true_word
    // find which characters in guess are in true_word but in the wrong place
    // find which characters in guess are not in true_word

    // make a string of the result
    // return the result

    let mut result = String::new();

    for i in 0..guess.len() {
        if guess[i] == true_word[i] {
            // result.push_str("\x1b[32m");
            let format_i = format_guess(guess[i], "green");
            // result.push_str("\x1b[0m");
        } else if true_word.contains(guess[i]) {
            // result.push_str("\x1b[33m");
            let format_i = format_guess(guess[i], "yellow");
            // result.push_str("\x1b[0m");
        } else {
            // result.push_str("\x1b[31m");
            let format_i = format_guess(guess[i], "grey");
            // result.push_str("\x1b[0m");
        }

        result.push(format_i);
    }



    return 0;
}



// make guesses 
// up to five guesses
// make guess using function 
// process guess
// if correct, return result and some stats about it 

// otherwise keep going in loop 
// if get to 6 guesses and still wrong, return result and stats as is 



// play wordle
pub fn play_wordle(true_word:Str) -> (i32) {
    let mut guess_count = 0;

    while guess_count < 7 {
        let guess = get_guess();
        let result = process_guess(guess, true_word);

        // print out result 
        println!("{result");


        guess_count += 1;
        if result == true_word {
            println!("You got it!");
            return 0;
        }
    }





    println!("{true_word");
    return 0;
}