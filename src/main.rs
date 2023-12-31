mod utils;
mod user;
mod ai;

use std::io;
use crate::ai::{AIStrategy, better_random, choose_ai_strategy, full_random, grundy};
use crate::user::player_move;
use crate::utils::{Board, test_grundy_performance};

fn main() {
    let mut input = String::new();

    println!("Test the performance of grundy function? (y/n)");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("y") {
        println!("Enter the max size: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(max_size) => test_grundy_performance(max_size),
            Err(_) => println!("Please type a number"),
        }
        return;
    }

    println!("Enter the board size: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let size: usize = match input.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Please type a number");
            return;
        }
    };

    let mut board = Board::new(size);
    board.display();

    let ai_strategy = choose_ai_strategy();

    loop {
        let (direction, steps) = match ai_strategy {
            AIStrategy::BetterRandom => better_random(&board),
            AIStrategy::FullRandom => full_random(&board),
            AIStrategy::Perfect => grundy(&board),
        };
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