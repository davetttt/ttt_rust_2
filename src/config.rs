use super::player::*;
use super::token::Token;
use super::io::*;

fn human_cpu<'a>() -> Vec<Box<Player + 'a>> {
    let mut players: Vec<Box<Player>> = Vec::new();
    players.push(Box::new(HumanPlayer::new(ConsoleIo::new(), Token::X)));
    players.push(Box::new(CpuPlayer::new(Token::O)));
    players
}

fn cpu_human<'a>() -> Vec<Box<Player + 'a>> {
    let mut players: Vec<Box<Player>> = Vec::new();
    players.push(Box::new(CpuPlayer::new(Token::X)));
    players.push(Box::new(HumanPlayer::new(ConsoleIo::new(), Token::O)));
    players
}

fn human_human<'a>() -> Vec<Box<Player + 'a>> {
    let mut players: Vec<Box<Player>> = Vec::new();
    players.push(Box::new(HumanPlayer::new(ConsoleIo::new(), Token::X)));
    players.push(Box::new(HumanPlayer::new(ConsoleIo::new(), Token::O)));
    players
}

pub fn configure_players<'a, I: Io>(io: &I) -> Vec<Box<Player + 'a>> {
    let config = io.prompt_with_options(concat!(
                "Chooose one of the following options:\n",
                "1) You vs. the game (you go first)\n",
                "2) The game vs. you (the game goes first)\n",
                "3) You vs. a friend\n"),
            vec!["1", "2", "3"]);

    match config {
        0 => human_cpu(),
        1 => cpu_human(),
        _ => human_human(),
    }
}

pub fn play_again<I: Io>(io: &I) -> bool {
    io.prompt_with_options("Play again? (y/n)\n", vec!["y", "n"]) == 0
}

