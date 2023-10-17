use std::io;
use crate::utils::{Board, Direction};

pub fn parse_input(input: &str) -> Option<(Direction, usize)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 2 {
        let direction = match parts[0].parse::<usize>() {
            Ok(0) => Direction::Left,
            Ok(1) => Direction::Down,
            Ok(2) => Direction::Diagonal,
            _ => return None,
        };
        let steps = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => return None,
        };
        Some((direction, steps))
    } else {
        None
    }
}

pub fn player_move(board: &Board) -> (Direction, usize) {
    loop {
        println!("Enter your move as two numbers separated by a space:");
        println!("0 => left, 1 => down, 2 => diagonal");
        println!("For example, '1 3' moves down 3 spaces.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Some((direction, steps)) = parse_input(&input) {
            if board.is_valid_move(direction, steps) {
                return (direction, steps);
            }
        } else {
            println!("Invalid input, try again.");
        }
    }
}