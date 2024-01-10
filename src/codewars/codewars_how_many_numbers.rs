// https://www.codewars.com/kata/5877e7d568909e5ff90017e6/train/rust
// 4 kyu

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

/*
fn increase_numbers(mut i: i32, mut lowest_digit: i32) -> (i32, i32) {
    let mut n = i;
    let mut last_digit = 0;
    while n > 0 {
        current_digit = n % 10; // Add the last digit to sum
        n /= 10; // Remove the last digit
    }

    i += 1;
    return (i, lowest_digit);
}
*/

/*
fn check_and_add_number(i: i32, sum_dig: u8) -> bool {
    let mut n = i;
    let mut sum = 0;
    let mut lowest_hi_digit = -1;
    while n > 0 {
        let current_digit: i32;
        if n >= 10 {
            current_digit = n % 10;
        } else {
            current_digit = n;
        }
        if current_digit > lowest_hi_digit && lowest_hi_digit != -1 {
            return false;
        }
        lowest_hi_digit = current_digit;
        sum += current_digit; // Add the last digit to sum
        n /= 10; // Remove the last digit
    }
    if sum == sum_dig as i32 {
        return true
    }
    return false;
}
*/

fn check_and_add_number(i: u32, sum_dig: u8) -> bool {
    let mut n = i;
    let mut sum = 0;
    let mut lowest_hi_digit = 10;  // Start higher than any digit

    while n > 0 {
        let current_digit = n % 10;
        if current_digit > lowest_hi_digit {
            return false;
        }
        lowest_hi_digit = current_digit;
        sum += current_digit;
        n /= 10;
    }

    if sum == sum_dig as u32 {
        return true;
    } else {
        return false;
    }
}

fn create_numbers(sum_dig: u8, digs: u8) -> Vec<u64>{
    let mut numbers = Vec::new();
    let mut stack = Vec::new();

    // Initialize stack
    for first_digit in 1..=9 {
        if first_digit <= sum_dig {
            stack.push((first_digit, first_digit as u64, 1));
        }
    }

    while let Some((sum, num, digit_count)) = stack.pop() {
        if digit_count > digs as usize {
            continue;
        }
        if digit_count == digs as usize && check_and_add_number(num as u32, sum_dig) {
            numbers.push(num as u64);
            continue;
        }

        for next_digit in 0..=9 {
            if sum + next_digit <= sum_dig {
                let new_num = num * 10 + next_digit as u64;
                stack.push((sum + next_digit, new_num, digit_count + 1));
            }
        }
    }
    numbers
}

fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    if sum_dig < 2 {
        return None;
    }
    // Still too inefficient. Instead:
    // 1. find lowest matching
    // 2. disperse that until done
    let mut numbers = create_numbers(sum_dig, digs);
    if numbers.is_empty() {
        return None;
    }
    numbers.sort_unstable();
    println!("{:?}", numbers);
    let count = numbers.len();
    let min = *numbers.first().unwrap();
    let max = *numbers.last().unwrap();
    Some((count, min, max))
}

pub fn run () {
    find_all(89, 9);
}

/* CODEWARS SOLUTIONS


*/