mod day01;

use std::io::{self, BufRead, BufReader};

fn load_input(day: usize) -> io::Result<impl BufRead> {
    use std::fs::File;
    use std::path::PathBuf;
    let path: PathBuf = ["input", &format!("day{:02}.txt", day)].iter().collect();
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
