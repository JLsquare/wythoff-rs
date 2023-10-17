use std::io;
use crate::utils::Direction;

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

pub fn get_player_move() -> (Direction, usize) {
    loop {
        println!("Enter your move as two numbers separated by a space:");
        println!("0 => left, 1 => down, 2 => diagonal");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Some((direction, steps)) = parse_input(&input) {
            return (direction, steps);
        } else {
            println!("Invalid input, try again.");
        }
    }
}