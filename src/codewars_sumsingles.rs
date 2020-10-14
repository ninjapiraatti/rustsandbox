// https://www.codewars.com/kata/59f11118a5e129e591000134/train/rust
// 7 kyu

// In this Kata, you will be given an array of numbers in 
// which two numbers occur once and the rest occur only twice. 
// Your task will be to return the sum of the numbers that occur only once.

// For example, repeats([4,5,7,5,4,8]) = 15 because only the n
// umbers 7 and 8 occur once, and their sum is 15.

#[cfg(test)]
mod tests {
    use super::repeats;

    #[test]
    fn basic_tests() {
        assert_eq!(repeats(&vec![4, 5, 7, 5, 4, 8]), 15);
        assert_eq!(repeats(&vec![9, 10, 19, 13, 19, 13]), 19);
        assert_eq!(repeats(&vec![16, 0, 11, 4, 8, 16, 0, 11]), 12);
        assert_eq!(repeats(&vec![5, 17, 18, 11, 13, 18, 11, 13]), 22);
        assert_eq!(repeats(&vec![5, 10, 19, 13, 10, 13]), 24);
    }
}

fn repeats(arr: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut ndl;
    let mut pos = None;
    let mut into_iter = arr.into_iter();
    for i in 0..arr.len() {
        ndl = i;
        pos = arr.iter().position(|&x| x == 100);
        println!("Find i in arr: {:?} {}", into_iter.find(| &x| x == &(i as i32)), i);
        //println!("{}", pos.is_none());
    }
    0
}

pub fn run () {
    repeats(&vec![9, 5, 2, 9, 2, 5, 8, 3]);
}

/* CODEWARS MOST UPVOTED
*/