use rand::Rng;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
pub(crate) enum State {
    ZERO,
    BASE,
    POST,
    ROOF,
    CORNER,
    ROPE,
    HUNG
}

impl State {
    pub(crate) fn toString(self) -> &'static str {
        match self {
            State::ZERO => "",
            State::BASE => "___",
            State::POST => " |\n |\n_|_",
            State::ROOF => " ________\n |\n |\n_|_",
            State::CORNER => "________\n |/\n |\n_|_",
            State::ROPE => "________\n |/      |\n |\n_|_",
            State::HUNG => "________\n |/      |\n |    L ö /\n_|_   /  /",
        }
    }

}

pub(crate) fn random_word() -> &'static str {

    let wordlist = memorable_wordlist::WORDS;
    let size:usize = wordlist.len();
    let num = rand::thread_rng().gen_range(0..size);

    println!("DEBUG: word: {}  number: {}",wordlist[num],num);;
    wordlist[num]


}

