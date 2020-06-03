pub mod grid2d;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

pub use grid2d::Grid2D;

use std::io;

pub fn load_input(day: usize) -> io::Result<String> {
    use std::fs::read_to_string;
    use std::path::PathBuf;
    let path: PathBuf = ["input", &format!("day{:02}.txt", day)].iter().collect();
    read_to_string(path)
}
