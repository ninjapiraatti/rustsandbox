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

fn find_numbers(lowest: Vec<i32>, starting_index: usize) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    let mut queue: Vec<Vec<i32>> = Vec::new();
    let mut operable = lowest;
    queue.push(operable.clone());
    let num = vec_to_u64(&operable);
    numbers.push(num);
    let length = operable.clone().len() - starting_index;
    let mut finished = false;

    while finished == false {
        operable = queue.first().unwrap().clone();
        let mut i = length;
        finished = true;
        while i > 0 {
            let mut j: i32 = i as i32 - 1;
            while j as i32 >= 0 {
                if operable[i] - operable[j as usize] > 1 {
                    operable[j as usize] += 1;
                    operable[i] -= 1;
                    if is_number_mutable(&operable, i, j as usize) == false {
                        if num_valid(&operable, length) == true {
                            let is_duplicate = queue.contains(&operable.clone());
                            let num = vec_to_u64(&operable);
                            if is_duplicate == false && !numbers.contains(&num) {
                                queue.push(operable.clone());
                            }
                            numbers.push(num);
                        }
                        operable = queue.first().unwrap().clone();
                        j -= 1;
                        finished = false;
                        continue;
                    }
                    let num = vec_to_u64(&operable);
                    let is_duplicate = queue.contains(&operable.clone());
                    if is_duplicate == false && !numbers.contains(&num) {
                        queue.push(operable.clone());
                        finished = false;
                    }
                    numbers.push(num);
                    operable = queue.first().unwrap().clone();
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
        numbers.extend(find_numbers(lowest.clone(), 1));
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

fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    dfs(sum_dig, digs, 1, 0)
    
}

fn dfs(target: u8, digits_remaining: u8,  min_digit: u8, number: u64) -> Option<(usize, u64, u64)> {
    if digits_remaining == 1 {
        if target < 10 && target >= min_digit {
            let number = number * 10 + target as u64;
            Some((1, number, number))
        } else {
            None
        }
    } else {
        let mut ret = (0, u64::MAX, 0);
        (min_digit..10.min(target)).for_each(|n| {            
            if let Some(branch_ret) = dfs(target-n, digits_remaining-1, n, number*10+(n as u64)) {
                ret.0 += branch_ret.0;
                ret.1 = ret.1.min(branch_ret.1);
                ret.2 = ret.2.max(branch_ret.2);
            }
        });
        if ret.0 > 0 {
            Some(ret)
        } else {
            None
        }        
    }
    
}

*/

/*

fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let mut count = 0usize;
    let mut min = 0u64;
    let mut max = 0u64;
    recurse_search(0, 1, sum_dig, digs, &mut count, &mut min, &mut max);
    if count > 0 { Some((count, min, max)) } else { None }
}

fn recurse_search(curr: u64, prev: u64, sum_left: u8, digs_left: u8, count: &mut usize, min: &mut u64, max: &mut u64) {
    if sum_left == 0 && digs_left == 0 {
        if *count == 0 { *min = curr; }
        if *min > curr { *min = curr; }
        if *max < curr { *max = curr; }
        *count += 1;
    } else if digs_left != 0 {
        for i in prev..10 {
            if i as u8 > sum_left { return; }
            recurse_search(10 * curr + i, i, sum_left - i as u8, digs_left - 1, count, min, max);
        }
    }
}

*/
