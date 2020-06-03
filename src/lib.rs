#[allow(dead_code)]
mod day01;

use std::io;

pub fn load_input(day: usize) -> io::Result<Vec<u8>> {
    use std::fs::read;
    use std::path::PathBuf;
    let path: PathBuf = ["input", &format!("day{:02}.txt", day)].iter().collect();
    read(path)
}
