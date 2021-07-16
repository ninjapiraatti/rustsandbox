// You get a list of integers, and you have to write a function
// mirror that returns the "mirror" (or symmetric) version of
// this list: i.e. the middle element is the greatest, then the
// next greatest on both sides, the the next greatest, and so on...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(mirror(&[]), &[]);
        assert_eq!(mirror(&[1]), &[1]);
        assert_eq!(mirror(&[2, 1]), &[1, 2, 1]);
        assert_eq!(mirror(&[2, 3, 1]), &[1, 2, 3, 2, 1]);
        assert_eq!(
            mirror(&[-8, 42, 18, 0, -16]),
            &[-16, -8, 0, 18, 42, 18, 0, -8, -16]
        );
    }
}

fn mirror(list: &[i32]) -> Vec<i32> {
    let mut res = list.to_vec();
    res.sort();
    res.reverse();
    for i in 1..list.len() {
        res.insert(0, res[i + i - 1]);
    }
    res
}

pub fn run() {
    mirror(&[2, 3, 1, 9, 0]);
}

/* CODEWARS MOST UPVOTED
fn mirror(xs: &[i32]) -> Vec<i32> {
    let mut res = xs.to_vec();
    res.sort();
    res.iter().chain(res.iter().rev().skip(1)).cloned().collect()
}
*/
