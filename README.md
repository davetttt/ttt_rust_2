# Rust Tic Tac Toe

## Install Rust
I used the 1.0.0-alpha version. Here's how I did it:
  1. Download the 1.0.0-alpha binaries [here][rust] (not the nightly build!)
  2. Extract the .tar
  3. `cd` into the new directory and run `sh install.sh`

By installing this way, you also get Cargo, the Rust build tool.

## Get the project
	git clone https://github.com/davetttt/ttt_rust.git
	cd ttt_rust

## Run the tests
	cargo test
* `cargo test` deletes the executable if one exists (so you'll have to `cargo build` again).

## Compile and play
	cargo build
	./target/ttt_rust

## Notes
* Several compilation warnings have been ignored by placing `#[allow(<warning-causer>)]` and an explanation above the offending functions. However, running the tests still warns that the program's `main` function is never used because there aren't any tests that call it. Placing `#[allow(dead_code)]` above `main` ignores warnings generated elsewhere so I decided against ignoring that warning.

[rust]: http://www.rust-lang.org/install.html
