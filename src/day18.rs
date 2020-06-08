use crate::load_input;
use crate::Grid2D;
use anyhow::{anyhow, Result};
use itertools::iproduct;
use std::fmt;

crate::aoc!(18);

pub fn day18_main() -> Result<()> {
    let input = load_input(18)?;
    let grid = parse_input(&input)?;

    let life1 = GameOfLife1 { grid: grid.clone() };
    assert_eq!(768, part1_solution(&life1, 100));
    let life2 = GameOfLife2 { grid };
    assert_eq!(781, part2_solution(&life2, 100));

    Ok(())
}

fn parse_input(input: &str) -> Result<Grid2D<bool>> {
    let data = input
        .lines()
        .map(|line| line.chars())
        .flatten()
        .map(|c| match c {
            '#' => Ok(true),
            '.' => Ok(false),
            c => Err(anyhow!("Unrecognized character: {:?}", c)),
        })
        .collect::<Result<Vec<bool>>>()?;

    let grid = Grid2D {
        data,
        width: input.lines().count(),
        height: input.lines().next().unwrap().len(),
    };

    Ok(grid)
}

#[derive(Clone)]
struct GameOfLife1 {
    grid: Grid2D<bool>,
}

impl Iterator for GameOfLife1 {
    type Item = Grid2D<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next_grid = Grid2D::new(self.grid.width, self.grid.height);
        //let mut next_grid = self.grid.clone();
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let neighbors = count_neighbors(x, y, &self.grid);
                match self.grid[(x, y)] {
                    true => {
                        if neighbors == 2 || neighbors == 3 {
                            next_grid[(x, y)] = true;
                        } else {
                            next_grid[(x, y)] = false;
                        }
                    }
                    false => {
                        if neighbors == 3 {
                            next_grid[(x, y)] = true;
                        } else {
                            next_grid[(x, y)] = false;
                        }
                    }
                }
            }
        }
        Some(std::mem::replace(&mut self.grid, next_grid))
    }
}

fn count_neighbors(x: usize, y: usize, grid: &Grid2D<bool>) -> usize {
    let mut neighbors = 0;
    for (dx, dy) in iproduct!(-1..2, -1..2) {
        if dx == 0 && dy == 0 {
            continue;
        }
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx >= 0 && nx < grid.width as isize && ny >= 0 && ny < grid.height as isize {
            match grid[(nx as usize, ny as usize)] {
                true => neighbors += 1,
                false => continue,
            }
        }
    }
    neighbors
}

impl fmt::Debug for Grid2D<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self[(x, y)] {
                        true => '#',
                        false => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1_solution(life: &GameOfLife1, count: usize) -> usize {
    let grid = life.clone().take(count + 1).last().unwrap();
    grid.iter().filter(|(_x, _y, &val)| val == true).count()
}

#[derive(Clone)]
struct GameOfLife2 {
    grid: Grid2D<bool>,
}

impl Iterator for GameOfLife2 {
    type Item = Grid2D<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next_grid = Grid2D::new(self.grid.width, self.grid.height);
        //let mut next_grid = self.grid.clone();
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let neighbors = count_neighbors(x, y, &self.grid);
                match self.grid[(x, y)] {
                    true => {
                        if neighbors == 2 || neighbors == 3 {
                            next_grid[(x, y)] = true;
                        } else {
                            next_grid[(x, y)] = false;
                        }
                    }
                    false => {
                        if neighbors == 3 {
                            next_grid[(x, y)] = true;
                        } else {
                            next_grid[(x, y)] = false;
                        }
                    }
                }
            }
        }
        next_grid[(0, 0)] = true;
        next_grid[(0, self.grid.height - 1)] = true;
        next_grid[(self.grid.width - 1, 0)] = true;
        next_grid[(self.grid.width - 1, self.grid.height - 1)] = true;
        Some(std::mem::replace(&mut self.grid, next_grid))
    }
}

fn part2_solution(life: &GameOfLife2, count: usize) -> usize {
    let mut life = life.clone();
    let width = life.grid.width;
    let height = life.grid.height;
    life.grid[(0, 0)] = true;
    life.grid[(0, height - 1)] = true;
    life.grid[(width - 1, 0)] = true;
    life.grid[(width - 1, height - 1)] = true;
    let grid = life.take(count + 1).last().unwrap();
    grid.iter().filter(|(_x, _y, &val)| val == true).count() + 1
}

#[test]
fn day18_examples() {
    let life = parse_input(
        "
.#.#.#
...##.
#....#
..#...
#.#..#
####.."
            .trim(),
    )
    .unwrap();
    assert_eq!(4, part1_solution(&GameOfLife1 { grid: life.clone() }, 4));
    assert_eq!(17, part2_solution(&GameOfLife2 { grid: life }, 5));
}
