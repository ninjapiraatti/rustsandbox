// Hands-On Data Structures and Algorithms in Rust
// Video 13: Quick sort

pub fn pivot<T: std::cmp::PartialOrd>(v: &mut [T]) -> usize {
	//let mut p = rng::rng(v.len()); // How to get the rng to work from here?
	let mut p = v.len() - 2;
	v.swap(p, 0);
	p = 0;
	for i in 1..v.len() {
		if v[i] < v[p] {
			// Move our pivot forward by 1, swap this element behind it
			v.swap(p + 1, i);
			v.swap(p, p + 1);
			p += 1;
		}
	}
	p 
}

pub fn quick_sort<T: std::cmp::PartialOrd + std::fmt::Debug>(v: &mut[T]) {
	if v.len() <= 1 {
		return;
	}
	let p = pivot(v);
	let (a, b) = v.split_at_mut(p);
	quick_sort(a);
	quick_sort(&mut b[1..]);
}

pub fn run () {
	let mut v = vec![1, 6, 4, 99, 0, -1, 7];
	quick_sort(&mut v);
	println!("{:?}", v);
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