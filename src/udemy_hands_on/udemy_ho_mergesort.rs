// Hands-On Data Structures and Algorithms in Rust
// Video 12: Merge sort

pub fn merge_sort<T: std::cmp::PartialOrd>(mut v: Vec<T>) -> Vec<T> {
    // Sort the left half,
    // Sort the right half,
    // Bring them together

    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b); // Shadowing

    // Bring together
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();
    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

pub fn run() {
    let v = vec![1, 6, 4, 99, 0, -1, 7];
    println!("{:?}", merge_sort(v));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(merge_sort(vec![1]), vec![1]);
        assert_eq!(merge_sort(vec![2, 1]), vec![1, 2]);
        assert_eq!(merge_sort(vec![2, 3, 1]), vec![1, 2, 3]);
        assert_eq!(
            merge_sort(vec![-8, 42, 18, 0, -16]),
            vec![-16, -8, 0, 18, 42]
        );
    }
}
