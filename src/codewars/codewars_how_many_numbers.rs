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

use std::time::Instant;

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

fn vec_to_u64(vec: &Vec<i32>) -> u64 {
    let mut result: u64 = 0;
    for &digit in vec {
        result = result * 10 + digit as u64;
    }
    result
}

fn is_number_mutable(vec: &Vec<i32>, current_index: usize, target_index: usize) -> bool {
    let mut k = current_index;
    let mut lowest;
    while k > target_index {
        lowest = vec[k];
        if vec[k - 1] >= lowest {
            return false;
        }
        k -= 1;
    }
    true
}

fn num_valid(vec: &Vec<i32>, len: usize) -> bool {
    let mut i = 0;
    while i < len {
        if vec[i + 1] < vec[i] {
            return false;
        }
        i += 1;
    }
    return true;
}

fn find_lowest_valid_number(sum: u8, digs: u8) -> Option<Vec<i32>> {
    let mut num = vec![0; digs as usize];
    let mut remainder = sum as i32;
    let mut end_index = 0;
    for i in 0..digs as usize {
        num[i] = 1;
        remainder -= 1;
        end_index += 1;
    }
    end_index -= 1;
    if remainder == 0 {
        return Some(num);
    }
    if remainder < 0 {
        return None;
    }
    while remainder > 0 {
        while num[end_index] < 9 && remainder > 0 {
            num[end_index] += 1;
            remainder -= 1;
        }
        if end_index > 0 {
            end_index -= 1;
        }
        if end_index == 0 && remainder > 8 {
            return None;
        }
    }
    return Some(num);
}

fn generate_numbers_two(lowest: Vec<i32>, starting_index: usize) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    let mut queue: Vec<Vec<i32>> = Vec::new();
    let mut base_string = lowest;
    queue.push(base_string.clone());
    let num = vec_to_u64(&base_string);
    numbers.push(num);
    let length = base_string.clone().len() - starting_index;
    let mut finished = false;

    while finished == false {
        base_string = queue.first().unwrap().clone();
        let mut i = length;
        finished = true;
        while i > 0 {
            let mut j: i32 = i as i32 - 1;
            while j as i32 >= 0 {
                if base_string[i] - base_string[j as usize] > 1 {
                    base_string[j as usize] += 1;
                    base_string[i] -= 1;
                    if is_number_mutable(&base_string, i, j as usize) == false {
                        if num_valid(&base_string, length) == true {
                            let is_duplicate = queue.contains(&base_string.clone());
                            let num = vec_to_u64(&base_string);
                            if is_duplicate == false && !numbers.contains(&num) {
                                queue.push(base_string.clone());
                            }
                            numbers.push(num);
                        }
                        base_string = queue.first().unwrap().clone();
                        j -= 1;
                        finished = false;
                        continue;
                    }
                    let num = vec_to_u64(&base_string);
                    let is_duplicate = queue.contains(&base_string.clone());
                    if is_duplicate == false && !numbers.contains(&num) {
                        queue.push(base_string.clone());
                        finished = false;
                    }
                    numbers.push(num);
                    base_string = queue.first().unwrap().clone();
                }
                j -= 1;
            }
            i -= 1;
        }
        queue.remove(0);
    }
    return numbers;
}

fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let mut numbers: Vec<u64> = Vec::new();
    if sum_dig < 2 {
        return None;
    }
    if let Some(lowest) = find_lowest_valid_number(sum_dig, digs) {
        numbers.extend(generate_numbers_two(lowest.clone(), 1));
    } else {
        return None;
    }
    numbers.sort_unstable();
    numbers.dedup();
    let count = numbers.len();
    let min = *numbers.first().unwrap();
    let max = *numbers.last().unwrap();
    Some((count, min, max))
}

pub fn run() {
    //find_all(10, 3);
    //find_all(27, 3);
    //find_all(84, 4);
    //find_all(35, 6);
    find_all(55, 11);
    //find_all(16, 4);
}

/* CODEWARS SOLUTIONS


*/
