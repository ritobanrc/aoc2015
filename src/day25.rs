use anyhow::Result;

crate::aoc!(25);

pub fn day25_main() -> Result<()> {
    //
    //   | 1   2   3   4   5   6
    //---+---+---+---+---+---+---+
    // 1 |  1   3   6  10  15  21
    // 2 |  2   5   9  14  20
    // 3 |  4   8  13  19
    // 4 |  7  12  18
    // 5 | 11  17
    // 6 | 16
    //
    // First, observe that the numbers across row 1 are triangle numbers.
    // The nth triangle number = n(n+1)/2  --- geometrically, this is the area formed by the
    // triangle.
    // Now, we need some way to convert from coordinates to indicies.
    // Consider the coordinate (4, 1). It contains the 4th triangle number.
    // Next, consider (4, 2) = 14. It is 1 before the 5th triangle number, 15
    // Next, consider (4, 3) = 19. It is 2 before the 6th triangle number, 21.
    // Generally, the coordinate (a, b) corresponds to b-1 before the (a + b - 1)th triangle number.
    // i.e. (a, b) -> (a + b - 1)(a + b)/2 - b + 1
    //
    // INPUT: To continue, please consult the code grid in the manual.  Enter the code at row 2978, column 3083.
    let input = (3083, 2978); // NOTE: I am storing coordinates as (x, y), or (column, row)
    let code_num = {
        let a = input.0;
        let b = input.1;
        let n = a + b - 1;
        n * (n + 1) / 2 - b + 1
    };
    assert_eq!(18361853, code_num);

    // Next, the codes are generated using an LCG (linear congruential generator)
    let mut code: u64 = 20151125;
    for _ in 0..code_num - 1 {
        code = (code * 252533) % 33554393;
    }
    assert_eq!(2650453, code);

    Ok(())
}
