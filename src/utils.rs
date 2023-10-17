
use std::time::Instant;
use rand::Rng;
use crate::ai::calculate_grundy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Down,
    Diagonal,
}

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    position: (usize, usize),
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        Board {
            size,
            position: (rng.gen_range(0..size), rng.gen_range(0..size)),
        }
    }

    pub fn display(&self) {
        for y in (0..self.size).rev() {
            for x in 0..self.size {
                print!("{} ", if self.position == (x, y) { "o" } else { "." });
            }
            println!();
        }
    }

    pub fn move_piece(&mut self, direction: Direction, steps: usize) -> bool {
        match direction {
            Direction::Left if self.position.0 >= steps => {
                self.position.0 -= steps;
                true
            }
            Direction::Down if self.position.1 >= steps => {
                self.position.1 -= steps;
                true
            }
            Direction::Diagonal if self.position.0 >= steps && self.position.1 >= steps => {
                self.position.0 -= steps;
                self.position.1 -= steps;
                true
            }
            _ => false,
        }
    }

    pub fn is_valid_move(&self, direction: Direction, steps: usize) -> bool {
        if steps == 0 {
            return false;
        }
        match direction {
            Direction::Left => self.position.0 >= steps,
            Direction::Down => self.position.1 >= steps,
            Direction::Diagonal => self.position.0 >= steps && self.position.1 >= steps,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}

pub fn test_grundy_performance(max_size: usize) {
    let mut size = 1;
    while size <= max_size {
        let start_time = Instant::now();
        let _ = calculate_grundy(size);
        let duration = start_time.elapsed().as_nanos();
        println!("Size: {}, Time (s): {}, Time (ns): {} nanoseconds", size, duration as f64 / 1_000_000_000f64, duration);
        size *= 2;
    }
}