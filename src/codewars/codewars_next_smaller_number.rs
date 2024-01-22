// https://www.codewars.com/kata/5659c6d896bc135c4c00021e/train/rust
// 4 kyu

// Write a function that takes a positive integer and returns the next smaller positive integer containing the same digits.

// For example:

// next_smaller(21) == Some(12)
// next_smaller(531) == Some(513)
// next_smaller(2071) == Some(2017)
// Return -1 (for Haskell: return Nothing, for Rust: return None), when there is no smaller number that contains the same digits. Also return -1 when the next smaller number with the same digits would require the leading digit to be zero.

// next_smaller(9) == None
// next_smaller(135) == None
// next_smaller(1027) == None  // 0721 is out since we don't write numbers with leading zeros
// some tests will include very large numbers.
// test data only employs positive integers.

use std::{time::Instant, cmp::min};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
    }
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

fn get_that_number(start_num: u64, min_num: Option<u64>, mut fast_index: usize, mut slow_index: usize) -> Option<u64> {
    let mut temp_min: u64;
    if fast_index == 0 && slow_index == 1 {
        println!("Stopping at {:?}", Some(min_num).unwrap());
        return None;
    }
    if min_num.is_none() == false {
        temp_min = Some(min_num).unwrap().unwrap();
    } else {
        temp_min = start_num;
    }
    //println!("temp_min: {:?} | fast_index: {:?} | slow_index: {:?}", temp_min, fast_index, slow_index);
    let mut digits = temp_min.to_string().chars().collect::<Vec<_>>();
    if digits[fast_index] > digits[slow_index] && digits[fast_index] != '0' {
        digits.swap(fast_index, slow_index);
        let result_str: String = digits.into_iter().collect();
        let res = result_str.parse::<u64>().unwrap_or(start_num);
        if min_num.is_none() || (res > temp_min && res < start_num) {
            temp_min = res;
        }
        //println!("temp_min: {:?} | fast_index: {:?} | slow_index: {:?} | res: {:?}", temp_min, fast_index, slow_index, res);
    }
    if fast_index == 0 && slow_index > 1 {
        slow_index -= 1;
        fast_index = slow_index - 1;
    } else if fast_index > 0 {
        println!("fi: {:?}", fast_index);
        fast_index -= 1;
    }

    println!("Starting new recursion with {:?}", temp_min);
    let better_number = get_that_number(start_num, Some(temp_min), fast_index, slow_index);
    if better_number.is_none() {
        println!("Stopping recursion at {:?}", temp_min);
        return Some(temp_min);
    }
    return None;
}

fn next_smaller_number(n: u64) -> Option<u64> {
    let start = Instant::now(); // Start timing
    if n < 10 {
        return None
    }
    let start_index = n.to_string().len() - 1;
    if let Some(result) = get_that_number(n, None, start_index - 1, start_index) {
        let duration = start.elapsed();
        println!("Result: {:?} | {:?}", result, duration);
        return Some(result);
    }
    let duration = start.elapsed();
    println!("No result | {:?}", duration);
    None
}

pub fn run() {
    next_smaller_number(74083);
    next_smaller_number(21);
    next_smaller_number(531);
    next_smaller_number(1027);
    next_smaller_number(2071);
}

/* CODEWARS SOLUTIONS


*/
