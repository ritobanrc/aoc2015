// clippy considers functions only called in tests to be dead code, but this entire crate is _just_
// tests, and I don't want a bunch of dead code warnings polluting my clippy.
#[allow(dead_code)]
mod day01;
mod day02;
mod day03;
mod day04;

use std::io;

pub fn load_input(day: usize) -> io::Result<String> {
    use std::fs::read_to_string;
    use std::path::PathBuf;
    let path: PathBuf = ["input", &format!("day{:02}.txt", day)].iter().collect();
    read_to_string(path)
}
