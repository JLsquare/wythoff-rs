use rand::Rng;
use crate::utils::{Board, Direction};

pub fn full_random(board: &Board) -> (Direction, usize) {
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

pub fn better_random(board: &Board) -> (Direction, usize) {
    if board.position().0 == 0 {
        return (Direction::Down, board.position().1);
    }
    if board.position().1 == 0 {
        return (Direction::Left, board.position().0);
    }
    if board.position().0 == board.position().1 {
        return (Direction::Diagonal, board.position().0);
    }
    loop {
        let (direction, steps) = full_random(board);
        if check_move(board, direction, steps) {
            return (direction, steps);
        }
    }
}

fn check_move(board: &Board, direction: Direction, steps: usize) -> bool {
    let mut new_board = board.clone();
    new_board.move_piece(direction, steps);
    match new_board.position() {
        (0, _) => false,
        (_, 0) => false,
        (x, y) if x == y => false,
        _ => true,
    }
}