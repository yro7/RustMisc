use std::ascii::AsciiExt;
use std::cmp::PartialEq;
use std::io;
use rand::{random, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::hangman::HangmanState::HUNG;

#[derive(EnumIter, Debug)]

// Represents how much the hangman is advanced

pub(crate) enum HangmanState {
    ZERO = 0,
    BASE = 1,
    POST = 2,
    ROOF = 3,
    CORNER = 4,
    ROPE = 5,
    HUNG = 6,
}


// Represents the general state of the game

struct GameState {
    hangman_state: HangmanState,
    answer: &'static str,
    remaining_guesses: i32,
    guessed_letters: Vec<char>,
    wrong_guesses: Vec<char>,

}

impl GameState {
    fn new() -> Self {
        GameState {
            hangman_state: HangmanState::ZERO,
            answer: random_word(),
            remaining_guesses: 7,
            guessed_letters: Vec::new(),
            wrong_guesses: Vec::new(),
        }
    }

    fn is_finished(&self) -> bool {
        // Game is finished if all letters in the answer are guessed
        self.answer.chars().all(|c| self.guessed_letters.contains(&c));
        // Or if the player don't have any answers left
        if self.remaining_guesses <= 0 { return true; }
        false
    }

    fn get_letter_input(&self) -> char {
        loop {
            let mut input = String::new();

            println!("Please enter a single letter.");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_lowercase();

            // Check if the input is a single ASCII character
            if input.len() != 1 {
                println!("You need to input a single character!");
                continue;
            }

            if !input.is_ascii() {
                println!("Please enter a valid ASCII character.");
                continue;
            }

            let input_char = input.chars().next().unwrap();

            if !self.has_been_guessed(&input_char) {
                return input_char;
            }

            println!("You already tried or guessed this letter, try another.");
        }
    }



    fn has_been_guessed(&self, letter:&char) -> bool {
        self.wrong_guesses.contains(letter) || self.guessed_letters.contains(letter)
    }

    fn get_string(&self) -> String {
        let mut res = String::new();

        for letter in self.answer.chars() {

            if self.guessed_letters.contains(&letter) {
                res.push(letter);
            }

            else {
                res += "_";
            }

        }

        res.trim_end().to_string()

    }
}

impl HangmanState {
    pub(crate) fn to_string(&self) -> &'static str {
        match self {
            HangmanState::ZERO => "",
            HangmanState::BASE => "___",
            HangmanState::POST => " |\n |\n_|_",
            HangmanState::ROOF => " ________\n |\n |\n_|_",
            HangmanState::CORNER => "________\n |/\n |\n_|_",
            HangmanState::ROPE => "________\n |/      |\n |\n_|_",
            HangmanState::HUNG => "________\n |/      |\n |    L ö /\n_|_   /  /",
        }
    }

}


pub(crate) fn play() {

    println!("Let's play Hangman, guess the word I'm thinking of!");
    let mut game_state : GameState = GameState::new();


    while(!game_state.is_finished()) {

        println!("Remaining guesses : {:?}",game_state.remaining_guesses);
        println!("{}",game_state.hangman_state.to_string());
        println!("word : {}",game_state.get_string());
        println!("Fails: : {:?}",game_state.wrong_guesses);

        println!("Input a letter : ");

        let letter:char = game_state.get_letter_input();
        if(game_state.answer.contains(&letter.to_string())) {
            game_state.guessed_letters.push(letter);
        }

        else {
            if(game_state.hangman_state)
            game_state.hangman_state = HangmanState::iter().get(game_state.hangman_state as usize + 1).expect("?");
            game_state.remaining_guesses -= 1;
        }
    }


    if(game_state.remaining_guesses > 0) {
        println!("Congratulations, you guessed the word!");
    }

    else {
        println!("You lose :( Wanna try again ?");
    }


}

pub(crate) fn random_word() -> &'static str {

    let wordlist = memorable_wordlist::WORDS;
    let size:usize = wordlist.len();
    let num = rand::thread_rng().gen_range(0..size);

    println!("DEBUG: word: {}  number: {}",wordlist[num],num);;
    wordlist[num]
}


