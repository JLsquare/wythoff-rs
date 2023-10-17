mod utils;
mod user;
mod ai;

extern crate rand;
use std::io;
use crate::ai::{better_random, full_random};
use crate::user::player_move;
use crate::utils::Board;

fn main() {
    println!("Enter the board size: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let size: usize = input.trim().parse().expect("Please type a number");

    let mut board = Board::new(size);
    board.display();

    loop {
        let (direction, steps) = better_random(&board);
        board.move_piece(direction, steps);
        println!("AI moved {:?} {}", direction, steps);
        board.display();

        if board.position() == (0, 0) {
            println!("AI wins!");
            break;
        }

        let (direction, steps) = player_move(&board);
        board.move_piece(direction, steps);
        println!("User moved {:?} {}", direction, steps);
        board.display();

        if board.position() == (0, 0) {
            println!("User win!");
            break;
        }
    }
}