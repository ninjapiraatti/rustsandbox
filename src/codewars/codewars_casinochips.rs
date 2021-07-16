// https://www.codewars.com/kata/5e0b72d2d772160011133654
// 6 kyu

// You are given three piles of casino chips: white, green and black chips:

// the first pile contains only white chips
// the second pile contains only green chips
// the third pile contains only black chips

// Each day you take exactly two chips of different colors and head to the casino.
// You can choose any color, but you are not allowed to take two chips of the same color in a day.

// You will be given an array representing the number of chips of each color and your
// task is to return the maximum number of days you can pick the chips. Each day you
// need to take exactly two chips.

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example_cases() {
        assert_eq!(solve(&[1, 1, 1]), 1);
        assert_eq!(solve(&[1, 2, 1]), 2);
        assert_eq!(solve(&[4, 1, 1]), 2);
        assert_eq!(solve(&[8, 2, 8]), 9);
        assert_eq!(solve(&[8, 1, 4]), 5);
        assert_eq!(solve(&[7, 4, 10]), 10);
        assert_eq!(solve(&[12, 12, 12]), 18);
        assert_eq!(solve(&[1, 23, 2]), 3);
    }
}

// Pretty brute force. There must be an elegant mathematical solution.

fn solve(arr: &[u32; 3]) -> u32 {
    let mut res = arr.to_vec();
    let mut days = 0;
    res.sort();
    while res[0] >= 10000 {
        res[1] -= 1000;
        res[2] -= 1000;
        days += 1000;
        res.sort();
    }
    while res[2] > 0 && res[1] > 0 {
        res[2] -= 1;
        if res[1] > 0 {
            res[1] -= 1;
        } else if res[0] > 0 {
            res[0] -= 1;
        }
        res.sort();
        days += 1;
    }
    days
}

pub fn run() {
    solve(&[24045, 24100, 24400]);
}

/* CODEWARS GOOD SOLUTIONS

fn solve(arr: &[u32; 3]) -> u32 {
    let mut chips = arr.to_vec();
    chips.sort();

    match chips[0] + chips[1] < chips[2] {
        true => chips[0] + chips[1],
        false => (chips[0] + chips[1] + chips[2]) / 2,
    }
}

*/
