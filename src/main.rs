#![allow(unstable)]

use io::Io; // so that we can call 'print' on our ConsoleIo

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
    let io = io::ConsoleIo::new();
    io.print("Welcome to Tic Tac Toe.\n");

    loop {
        let players = config::configure_players(&io);
        let board = board::generate_empty_board(3);
        game::play_game(&players, &board, &io);

        if !config::play_again(&io) {
            break;
        }
    }
}

