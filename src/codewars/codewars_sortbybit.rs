// https://www.codewars.com/kata/52f5424d0531259cfc000d04
// 7 kyu

// Write a function which accepts a sequence of unique integers as argument 
// and returns a 32-bit integer such that the integer, in its binary 
// representation has 1 at only those indexes (counted from right) which 
// are in the sequence.

// Examples
// sort_by_bit(&[0, 1]) // Returns integer 3
// sort_by_bit(&[1, 2, 0, 4]) // Returns integer 23  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            sort_by_bit(&[] as &[u8; 0]),
            0b0000,
            "should return 0 if empty array is provided",
        );
        
        assert_eq!(
            sort_by_bit(&[0]),
            0b0001,
            "should return 1 if array with zero is provided",
        );
                
        assert_eq!(
            sort_by_bit(&[0, 1]),
            0b0011,
            "should return 3 if array with zero and 1 is provided",            
        );
        
        assert_eq!(
            sort_by_bit(&[1, 0]),
            0b0011,
            "should return 3 if array with zero and 1 is provided, order shouldn't matter",
        );
        
        assert_eq!(
            sort_by_bit(&[30, 0]),
            0b0100_0000_0000_0000_0000_0000_0000_0001,
            "should return 1073741825 if array with 30 and 0 provided"
        );
    }
}

fn sort_by_bit(list: &[u8]) -> u32 {
    let mut res = 0;
    for i in list.iter() {
        res = res | (1 << i);
    }
    res
}

pub fn run () {
    sort_by_bit(&[0, 1]);
}

/* CODEWARS SOLUTIONS

fn sort_by_bit(xs: &[u8]) -> u32 {
    xs.iter().map(|x| 1 << x).sum()
}

fn sort_by_bit(list: &[u8]) -> u32 {
    let mut result = 0u32;
    for index in list {
        result |= 1 << index;
    }
    result
}

*/