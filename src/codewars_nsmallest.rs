// https://www.codewars.com/kata/5aec1ed7de4c7f3517000079
// 6 kyu

// Your task is to write a function that does just what the title suggests 
// (so, fair warning, be aware that you are not getting out of it just throwing 
// a lame bas sorting method there) with an array/list/vector of integers
// and the expected number n of smallest elements to return.

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
    //let mut sorted = &arr;
    let mut sorted = arr.to_vec();
    sorted.sort();
    println!("{:?}", sorted[0..n].to_vec());
    sorted[0..n].to_vec()
}

pub fn run () {
    n_smallest(&[0, 1, 1, 5, 6, 8, 3, 4, 10, -5], 2);
}

/* CODEWARS SOLUTIONS

*/