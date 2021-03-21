// Hands-On Data Structures and Algorithms in Rust
// Video 13: Quick sort

pub fn quick_sort<T: std::cmp::PartialOrd>(mut v: Vec<T>) -> Vec<T> {

}

pub fn run () {
	let v = vec![1, 6, 4, 99, 0, -1, 7];
	println!("{:?}", quick_sort(v));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(quick_sort(vec![1]), vec![1]);
        assert_eq!(quick_sort(vec![2, 1]), vec![1, 2]);
        assert_eq!(quick_sort(vec![2, 3, 1]), vec![1, 2, 3]);
        assert_eq!(quick_sort(vec![-8, 42, 18, 0, -16]), vec![-16, -8, 0, 18, 42]);
    }
}