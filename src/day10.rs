use anyhow::Result;

crate::aoc!(10);

pub fn day10_main() -> Result<()> {
    const INPUT: usize = 1321131112;
    let input: Vec<_> = digits(INPUT).collect();

    assert_eq!(492982, part1_solution(&input));
    assert_eq!(6989950, part2_solution(&input));

    Ok(())
}

fn part1_solution(digits: &[usize]) -> usize {
    let mut digits = digits.to_vec();
    for _ in 0..40 {
        digits = play_game(&digits);
    }
    digits.len()
}

fn part2_solution(digits: &[usize]) -> usize {
    let mut digits = digits.to_vec();
    for _ in 0..50 {
        digits = play_game(&digits);
    }
    digits.len()
}

fn play_game(digits: &[usize]) -> Vec<usize> {
    let mut next = Vec::new();
    let mut idx = 1;
    let mut prev = digits[0];
    let mut run_len = 1;
    while idx < digits.len() {
        if prev == digits[idx] {
            run_len += 1;
        } else {
            next.push(run_len);
            next.push(prev);
            prev = digits[idx];
            run_len = 1;
        }
        idx += 1;
    }
    next.push(run_len);
    next.push(prev);
    next
}

fn digits(mut num: usize) -> impl Iterator<Item = usize> {
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }

    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}

#[test]
fn day10_examples() {
    assert_eq!(play_game(&[1]), vec![1, 1]);
    assert_eq!(play_game(&[1, 1]), vec![2, 1]);
    assert_eq!(play_game(&[2, 1]), vec![1, 2, 1, 1]);
    assert_eq!(play_game(&[1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
    assert_eq!(play_game(&[1, 1, 1, 2, 2, 1]), vec![3, 1, 2, 2, 1, 1]);
}
