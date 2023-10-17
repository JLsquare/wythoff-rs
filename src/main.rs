mod utils;
mod user;
mod ai;

extern crate rand;
use std::io;
use crate::ai::random_ai;
use crate::user::get_player_move;
use crate::utils::Board;

fn main() {
    println!("Enter the board size: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let size: usize = input.trim().parse().expect("Please type a number");

    let mut board = Board::new(size);
    board.display();

    loop {
        loop {
            let (direction, steps) = random_ai(&board);
            if board.move_piece(direction, steps) {
                println!("AI moved {:?} {}", direction, steps);
                board.display();
                break;
            } else {
                println!("Invalid move by AI, trying again...");
            }
        }

        if board.position() == (0, 0) {
            println!("AI wins!");
            break;
        }

        loop {
            let (direction, steps) = get_player_move();
            if board.move_piece(direction, steps) {
                println!("User moved {:?} {}", direction, steps);
                board.display();
                break;
            } else {
                println!("Invalid move, try again...");
            }
        }

        if board.position() == (0, 0) {
            println!("User win!");
            break;
        }
    }
}