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

        // 4 2 1 > 3 1 1 > 2 0 1 > 1 0 0                                                   3 7
        // 7 3 1 > 6 2 1 > 5 1 1 > 4 0 1 > 3 0 0                                           4 11 
        // 9 8 2 > 8 7 2 > 7 6 2 > 6 5 2 > 5 4 2 > 4 3 2 > 3 2 2 > 2 1 2 > 1 1 1 > 0 0 1   9 19
        // 5 5 0 > 4 4 0 > 3 3 0 > 2 2 0 > 1 1 0 > 0 0 0                                   5 10
        // 8 8 2 > 7 7 2 > 6 6 2 > 5 5 2 > 4 4 2 > 3 3 2 > 2 1 2 > 1 1 1 > 0 0 1           8 18
        // 6 3 1 > 5 2 1 > 4 1 1 > 3 1 0 > 2 0 0                                           4 10 
    }
}

fn solve(arr: &[u32; 3]) -> u32 {
    let mut res = arr.to_vec();
    let mut days = 0;
    res.sort();
    while res[0] >= 12000 {
        res[0] -= 12000;
        res[1] -= 12000;
        res[2] -= 12000;
        days += 18000;
    }
    while res[2] > 0 && res[1] > 0{
        res.sort();
        res[2] -= 1;
        if res[1] > 0 {
            res[1] -= 1;
        } else if res[0] > 0 {
            res[0] -= 1;
        }
        days += 1;
    }
    println!("{:?}", (res, days));
    days
}

pub fn run () {
    solve(&[45, 100, 400]);
}

/* CODEWARS GOOD SOLUTIONS



*/