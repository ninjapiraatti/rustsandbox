// https://www.codewars.com/kata/5aec1ed7de4c7f3517000079
// 6 kyu

// Your task is to write a function that does just what the title suggests 
// (so, fair warning, be aware that you are not getting out of it just throwing 
// a lame bas sorting method there) with an array/list/vector of integers
// and the expected number n of smallest elements to return.

// Sort by tuple 2nd element: https://stackoverflow.com/questions/40091161/sorting-a-vector-of-tuples-needs-a-reference-for-the-second-value

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(n_smallest(&[1,2,3,4,5],3), [1,2,3]);
        assert_eq!(n_smallest(&[5,4,3,2,1],3), [3,2,1]);
        assert_eq!(n_smallest(&[1,2,3,1,2],3), [1,2,1]);
        assert_eq!(n_smallest(&[1,2,3,-4,0],3), [1,-4,0]);
        assert_eq!(n_smallest(&[1,2,3,4,5],0), []);
        assert_eq!(n_smallest(&[1,2,3,4,5],5), [1,2,3,4,5]);
        assert_eq!(n_smallest(&[1,2,3,4,2],4), [1,2,3,2]);
        assert_eq!(n_smallest(&[2,1,2,3,4,2],2), [2,1]);
        assert_eq!(n_smallest(&[2,1,2,3,4,2],3), [2,1,2]);
        assert_eq!(n_smallest(&[2,1,2,3,4,2],4), [2,1,2,2]);
    }
}


fn n_smallest(arr: &[i32], n: usize) -> Vec<i32> {
    let arrtovec = arr.to_vec();
    let mut numbers = Vec::new(); // Has to be vec because array length must be known runtime
    for (pos, e) in arrtovec.iter().enumerate() {
        numbers.push((e, pos));
    }
    numbers.sort();
    numbers = numbers[0..n].to_vec();
    numbers.sort_by_key(|k| k.1);
    let mut res = Vec::new();
    for i in numbers {
        res.push(*i.0);
    }
    res
}

pub fn run () {
    n_smallest(&[0, 1, 1, 5, 6, 8, 3, 4, 10, -5], 8);
}

/* CODEWARS SOLUTIONS

use itertools::Itertools;

fn first_n_smallest(arr: &[i32], n: usize) -> Vec<i32> {
    arr.iter()
        .enumerate()
        .sorted_by_key(|(_, &c)| c)
        .take(n)        
        .sorted_by_key(|(i, _)| *i)
        .map(|(_, &c)| c)
        .collect()
}

*/