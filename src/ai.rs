use std::io;
use rand::Rng;
use crate::utils::{Board, Direction};

pub enum AIStrategy {
    BetterRandom,
    FullRandom,
    Perfect,
}

pub fn choose_ai_strategy() -> AIStrategy {
    loop {
        println!("Choose AI strategy: 1 for Better Random, 2 for Full Random, 3 for Perfect");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "1" => return AIStrategy::BetterRandom,
            "2" => return AIStrategy::FullRandom,
            "3" => return AIStrategy::Perfect,
            _ => println!("Invalid choice, try again"),
        }
    }
}

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

pub fn grundy(board: &Board) -> (Direction, usize) {
    let grundy_numbers = calculate_grundy(board.size());
    let (x, y) = board.position();

    for steps in 1..=x {
        let new_x = x - steps;
        if grundy_numbers[new_x][y] == 0 {
            return (Direction::Left, steps);
        }
    }

    for steps in 1..=y {
        let new_y = y - steps;
        if grundy_numbers[x][new_y] == 0 {
            return (Direction::Down, steps);
        }
    }

    let steps = std::cmp::min(x, y);
    if steps > 0 && grundy_numbers[x - steps][y - steps] == 0 {
        return (Direction::Diagonal, steps);
    }

    better_random(board)
}

pub fn calculate_grundy(n: usize) -> Vec<Vec<usize>> {
    let mut grundy = vec![vec![0; n + 1]; n + 1];

    fn mex(set: &Vec<usize>) -> usize {
        let mut i = 0;
        while set.contains(&i) {
            i += 1;
        }
        i
    }

    for a in 0..=n {
        for b in 0..=n {
            let mut set = Vec::new();
            for i in 0..a {
                set.push(grundy[i][b]);
            }
            for i in 0..b {
                set.push(grundy[a][i]);
            }
            for i in 1..=std::cmp::min(a, b) {
                set.push(grundy[a - i][b - i]);
            }
            grundy[a][b] = mex(&set);
        }
    }

    grundy
}