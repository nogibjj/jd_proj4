use owo_colors::{
    colors::{xterm::Gray, Black, Green, Yellow},
    OwoColorize,
};
use std::fmt;

mod guess;
use guess::get_guess;
use guess::make_first_guess;

fn format_guess(d: impl fmt::Display, color: String) -> String {
    if color == "green" {
        let res = format!("{}", d.fg::<Black>().bg::<Green>());
        res
    } else if color == "yellow" {
        let res = format!("{}", d.fg::<Black>().bg::<Yellow>());
        res
    } else {
        let res = format!("{}", d.fg::<Black>().bg::<Gray>());
        res
    }
}

pub fn process_guess(guess: String, true_word: &&str, word_list: Vec<String>) -> Vec<String> {
    // find which characters in guess are the same as true_word
    // find which characters in guess are in true_word but in the wrong place
    // find which characters in guess are not in true_word

    // make a string of the result
    // return the result

    let mut result = Vec::new();
    let mut progress = Vec::new();

    for i in 0..guess.len() {
        let guess_i = guess.as_bytes()[i] as char;
        let true_i = true_word.as_bytes()[i] as char;

        if guess_i == true_i {
            // result.push_str("\x1b[32m");
            let format_i = format_guess(guess_i.to_ascii_uppercase(), "green".to_string());
            progress.push("1");
            result.push(format_i);
            // result.push_str("\x1b[0m");
        } else if true_word.contains(guess_i) {
            // result.push_str("\x1b[33m");
            let format_i = format_guess(guess_i.to_ascii_uppercase(), "yellow".to_string());
            // result.push_str("\x1b[0m");
            progress.push("2");
            result.push(format_i);
        } else {
            // result.push_str("\x1b[31m");
            let format_i = format_guess(guess_i.to_ascii_uppercase(), "grey".to_string());
            // result.push_str("\x1b[0m");
            progress.push("0");
            result.push(format_i);
        }
    }

    // make result_str into a string
    let result_str = result.join("");
    let progress_str = progress.join("");

    println!("{result_str}");

    // filter word list based on progress
    // println!("{progress_str}");

    word_list
}



// play wordle until solved
pub fn play_wordle(true_word: &&str, word_list: Vec<String>) -> i32 {
    let mut guess_count = 0;
    // let mut guesses = Vec::new();
    // let mut results = Vec::new();

    let mut solved = false;
    let mut guess = String::new();

    let mut filtered_word_list = word_list;

    while !solved {
        if guess_count == 0 {
            guess = make_first_guess();
        } else {
            guess = get_guess(filtered_word_list.clone());
        }
        guess_count += 1;
        // process guess
        filtered_word_list = process_guess(guess.clone(), true_word, filtered_word_list.clone());

        if guess.clone() == *true_word.to_string() {
            println!(
                "Successfully solved for {} \n Total guesses required : {guess_count}",
                true_word.to_ascii_uppercase()
            );
            solved = true;
        }
    }
    guess_count
}
