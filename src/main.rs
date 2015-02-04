#![allow(unstable)]

#[cfg(test)]
mod tests;

mod board;
mod token;
mod io;
mod player;
mod rules;
mod minimax;
mod game;
mod config;

fn main() {
    loop {
        let io = io::ConsoleIo::new();
        let players = config::configure_players(&io);
        let board = board::generate_empty_board(3);
        game::game_loop(&players, &board);

        if !config::play_again(&io) {
            break;
        }
    }
}

