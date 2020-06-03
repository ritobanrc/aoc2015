use md5;

crate::aoc!(04);

pub fn day04_main() -> anyhow::Result<()> {
    const INPUT: &'static str = "ckczppom";

    assert_eq!(117946, part1_solution(INPUT));
    assert_eq!(3938038, part2_solution(INPUT));

    Ok(())
}

fn crack_md5(input: &str, starts_with: &str) -> usize {
    let mut ans = 1;
    loop {
        let data = format!("{}{}", input, ans);
        let digest = format!("{:x}", md5::compute(data));
        if digest.starts_with(starts_with) {
            return ans;
        }
        ans += 1;
    }
}

fn part1_solution(input: &str) -> usize {
    crack_md5(input, "00000")
}

fn part2_solution(input: &str) -> usize {
    crack_md5(input, "000000")
}
