// https://www.codewars.com/kata/59f061773e532d0c87000d16
// 7 kyu

// Imagine you start on the 5th floor of a building, then travel down to the 2nd floor, 
// then back up to the 8th floor. You have travelled a total of 3 + 6 = 9 floors of distance.

// Given an array representing a series of floors you must reach by elevator, return an integer 
// representing the total distance travelled for visiting each floor in the array in order.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(elevator_distance(&[5, 2, 8]), 9);
        assert_eq!(elevator_distance(&[1, 2, 3]), 2);
        assert_eq!(elevator_distance(&[7, 1, 7, 1]), 18);
    }
}

fn elevator_distance(floors: &[i16]) -> i16 {
	let mut res = 0i16;
	for window in floors.windows(2) {
		res += (window[0] - window[1]).abs();
		println!("{:?}", res);
    }
	res
}

pub fn run () {
    elevator_distance(&[1, 6, 1]);
}

/* CODEWARS GOOD SOLUTIONS

fn elevator_distance(floors: &[i16]) -> i16 {
    floors.windows(2).map(|s| (s[0] - s[1]).abs()).sum()
}

fn elevator_distance(floors: &[i16]) -> i16 {
    let mut distance = 0;

    for i in 1..floors.len() {
        distance += (floors[i] - floors[i - 1]).abs();
    }

    return distance;
}

*/