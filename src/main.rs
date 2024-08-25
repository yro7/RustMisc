use strum::IntoEnumIterator;
mod rock_paper_scissors;
mod fibonnaci;
mod hangman;

#[derive(Debug)]
struct Integer  {
    value:i32
}

fn main() {


    for state in hangman::State::iter(){
        println!("{}", state.toString());
    }

}