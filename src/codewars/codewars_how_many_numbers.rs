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

fn vec_to_u64(vec: &Vec<i32>) -> u64 {
    let mut result: u64 = 0;
    for &digit in vec {
        result = result * 10 + digit as u64;
    }
    result
}

fn find_lowest_valid_number(sum: u8, digs: u8) -> Option<Vec<i32>> {
    let mut num = vec![0; digs as usize];  // Vector to store digits of the number

    // Initialize the smallest number
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
        // error
        return None;
    }
    //println!("{:?}", remainder);
    while remainder > 0 {
        while num[end_index] < 9 && remainder > 0 {
            num[end_index] += 1;
            remainder -= 1;
        }
        if end_index > 0 {
            end_index -= 1;
        }
        if end_index == 0 && remainder > 8 {
            // error
            println!("{:?}", remainder);
            return None;
        }
    }
    //println!("{:?}", num);
    return Some(num);
}

fn generate_numbers(lowest: Vec<i32>) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    let mut base_string = lowest;
    let mut i = base_string.len() - 1;
    let mut strings: Vec<Vec<i32>> = Vec::new();
    let mut j = 1;
    let mut mutated = false;

    //println!("Starting loop with {:?}", base_string);
    while i > 0 {
        while j <= i && base_string[i] - base_string[i - j] > 1 {
            base_string[i - j] += 1;
            base_string[i] -= 1;
            let num = vec_to_u64(&base_string);
            if base_string[(i - j) + 1] >= base_string[i - j] {
                //println!("Saving {:?}", num);
                strings.push(base_string.clone());
                numbers.push(num);
                mutated = true;
            }
            if base_string[i] <= base_string[i -j] {
                break
            }
            j += 1;
        }
        i -= 1;
    }
    if mutated == false {
        return numbers;
    }
    for num in strings {
        let new_numbers = generate_numbers(num);
        numbers.extend(new_numbers);
    }
    //println!("Returning {:?}", numbers);
    return numbers;
    /*
    let mut numbers: Vec<u64> = Vec::new();
    let mut string = lowest;
    //numbers.push(vec_to_u64(&string));
    let i = string.len() - 1;
    let mut finished = false;
    while !finished {
        let mut j = i;
        let mut temp_string = string.clone();
        let mut k = j - 1;
        while j > 0 {
            while j > 0 && temp_string[j] - temp_string[k] > 1 {
                let prev_string = temp_string.clone();
                println!("\n\nSTART: temp_string: {:?} \nprev_string: {:?} \nj: {:?} | k: {:?}", temp_string, prev_string, j, k);
                temp_string[k] += 1;
                temp_string[j] -= 1;
                //println!("temp_string is now: {:?}", temp_string);
                let num = vec_to_u64(&temp_string);
                println!("\n\n{:?} | {:?}", num, numbers);
                if temp_string[j] - temp_string[k] < 2 && k > 0{
                    println!("\n\nLast iteration!");
                    println!("saving: {:?}", temp_string);
                    numbers.push(vec_to_u64(&temp_string));
                    k -= 1;
                    j -= 1;
                    temp_string = prev_string.clone();
                } else {
                    println!("saving: {:?}", temp_string);
                    numbers.push(vec_to_u64(&temp_string));
                }
                println!("\n\nEND: temp_string: {:?} \nprev_string: {:?} \nj: {:?} | k: {:?}", temp_string, prev_string, j, k);
            }
            /*
            j -= 1;
            if j > 0 {
                k = j - 1;
            }*/
            //println!("string is now: {:?}", string);
            /*
            if mutated {
                j = i;
                mutated = false;
            } else {
                j -= 1;
            }
            */
        }
        finished = true;
    }
    return numbers;
    */
}

fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let mut numbers: Vec<u64>;
    if sum_dig < 2 {
        return None;
    }
    if let Some(lowest) = find_lowest_valid_number(sum_dig, digs) {
        numbers = generate_numbers(lowest);
    } else {
        return None;
    }
    numbers.sort_unstable();
    numbers.dedup();
    let count = numbers.len();
    println!("{:?} | {:?}", numbers, count);
    let min = *numbers.first().unwrap();
    let max = *numbers.last().unwrap();
    Some((count, min, max))
}

pub fn run () {
    //find_all(10, 3);
    //find_all(27, 3);
    //find_all(84, 4);
    //find_all(35, 6);
    //find_all(77, 11);
    find_all(16, 4);
}

/* CODEWARS SOLUTIONS


*/