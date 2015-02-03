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

fn main() {
    println!("Tic tac toe");
}

