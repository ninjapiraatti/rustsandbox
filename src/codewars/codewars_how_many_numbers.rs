// We want to generate all the numbers of three digits where:

// the sum of their digits is equal to 10
// their digits are in increasing order (the numbers may have two or more equal contiguous digits)
// The numbers that fulfill these constraints are: [118, 127, 136, 145, 226, 235, 244, 334]. There're 8 numbers in total with 118 being the lowest and 334 being the greatest.

// Task
// Implement a function which receives two arguments:

// the sum of digits (sum)
// the number of digits (count)
// This function should return three values:

// the total number of values which have count digits that add up to sum and are in increasing order
// the lowest such value
// the greatest such value
// Note: if there're no values which satisfy these constaints, you should return an empty value (refer to the examples to see what exactly is expected).

#[cfg(test)]
mod tests {
    use super::find_all;

    #[test]
    fn sample_tests() {
        assert_eq!(find_all(10, 3), Some((8, 118, 334)));
        assert_eq!(find_all(27, 3), Some((1, 999, 999)));
        assert_eq!(find_all(84, 4), None);
        assert_eq!(find_all(35, 6), Some((123, 116999, 566666)));
    }
}

fn increase_numbers(mut i: i32, mut lowest_digit: i32) -> (i32, i32) {
    i += 1;
    return (i, lowest_digit);
}

fn check_and_add_number(i: i32, sum_dig: u8) -> Option<i32> {
    let mut n = i;
    let mut sum = 0;
    while n > 0 {
        sum += n % 10; // Add the last digit to sum
        n /= 10; // Remove the last digit
    }
    //println!("{:?}", sum);
    if sum == sum_dig as i32 {
        return Some(i);
    }
    return None;
}

fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    if sum_dig < 2 {
        return None;
    }
    let starting_number = (10 as i32).pow((digs - 1) as u32);
    let max_number = (starting_number * 10) - starting_number - (starting_number / 10);
    let mut numbers: Vec<i32> = Vec::new();
    let mut lowest_digit = 1;
    let mut i = starting_number;
    while i <= max_number {
        //i += lowest_digit;
        (i, lowest_digit) = increase_numbers(i, lowest_digit);
        if let Some(needle) = check_and_add_number(i, sum_dig) {
            numbers.push(needle);
        }
    }
    println!("{:?}, | {:?} | {:?}", i, lowest_digit, numbers);

    return None;
}

pub fn run () {
    find_all(25, 4);
}

/* CODEWARS SOLUTIONS


*/