// https://www.codewars.com/kata/5f70e4cce10f9e0001c8995a/train/rust
// 7 kyu

// There are some stones on Bob's table in a row, and each of them can be red, 
// green or blue, indicated by the characters R, G, and B.

// Help Bob find the minimum number of stones he needs to remove from the 
// table so that the stones in each pair of adjacent stones have different colours.

#[cfg(test)]
mod tests {
    use super::stones_to_remove;

    #[test]
    fn sample_tests() {
        assert_eq!(stones_to_remove("RGBRGBRGGB"), 1);
        assert_eq!(stones_to_remove("RGGRGBBRGRR"), 3);
        assert_eq!(stones_to_remove("RRRRGGGGBBBB"), 9);
    }
}



fn stones_to_remove(stones: &str) -> usize {
    let mut sum = 0;
    let mut temp = ' ';
    for c in stones.chars() {
        if c == temp {
            sum += 1;
        } 
        temp = c;
    }
    sum
}

pub fn run () {
    stones_to_remove("RRGRRG");
}

/* CODEWARS MOST UPVOTED

fn stones_to_remove(stones: &str) -> usize {
    stones.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| x[0] == x[1])
        .count()
}

*/