// https://www.codewars.com/kata/56a5d994ac971f1ac500003e
// 6 kyu

// You are given an array(list) strarr of strings and an integer k. 
// Your task is to return the first longest string consisting of k consecutive strings taken in the array.

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1, 
        "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut sorted_strings = strarr.sort_by(|s| s.len());
}

pub fn run() {
    longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2);
}

/* CODEWARS GOOD SOLUTIONS


*/
