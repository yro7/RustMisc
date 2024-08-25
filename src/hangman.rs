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

