use rand::Rng;
use crate::utils::{Board, Direction};

pub fn random_ai(board: &Board) -> (Direction, usize) {
    let mut rng = rand::thread_rng();
    loop {
        let direction = match rng.gen_range(0..3) {
            0 => Direction::Left,
            1 => Direction::Down,
            _ => Direction::Diagonal,
        };
        let steps = rng.gen_range(1..board.size());
        if board.is_valid_move(direction, steps) {
            return (direction, steps);
        }
    }
}