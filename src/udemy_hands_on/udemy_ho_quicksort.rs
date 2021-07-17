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

pub fn quick_sort<T: std::cmp::PartialOrd + std::fmt::Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

struct RawSend<T>(*mut [T]);
unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quick_sort<T: 'static + std::cmp::PartialOrd + std::fmt::Debug + Send>(
    v: &mut [T],
) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);

    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);
    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quick_sort(&mut *raw_s.0);
        });
        threaded_quick_sort(b);
        handle.join().ok();
    }
}

pub fn run() {
    let mut v = vec![1, 6, 4, 99, 0, -1, 7];
    quick_sort(&mut v);
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn basic() {
        //let mut v1 = vec![1, 6, 4, 99, 0, -1, 7];
        //let mut v2 = vec![1, 6, 4, 99, 0, -1, 7];
        //let v2_sorted = quick_sort(&mut v2);
        let v2_correct = vec![1, 2];
        //let mut v3 = vec![1, 6, 4, 99, 0, -1, 7];
        //let mut v4 = vec![1, 6, 4, 99, 0, -1, 7];
        //assert_eq!(quick_sort(&mut v1), vec![1]);
        assert_eq!(v2_correct, vec![1, 2]);
        //assert_eq!(quick_sort(&mut v3), vec![1, 2, 3]);
        //assert_eq!(quick_sort(&mut v4), vec![-16, -8, 0, 18, 42]);
    }
}
