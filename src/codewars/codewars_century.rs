// https://www.codewars.com/kata/52fb87703c1351ebd200081f/train/rust
// 6 kyu

// Return the century of the input year. The input will always be a
// 4 digit string, so there is no need for validation.
// Examples
// "1999" --> "20th"
// "2011" --> "21st"
// "2154" --> "22nd"
// "2259" --> "23rd"
// "1124" --> "12th"
// "2000" --> "20th"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(what_century("1999"), "20th");
        assert_eq!(what_century("2011"), "21st");
        assert_eq!(what_century("2154"), "22nd");
        assert_eq!(what_century("2259"), "23rd");
        assert_eq!(what_century("1234"), "13th");
        assert_eq!(what_century("1023"), "11th");
        assert_eq!(what_century("2000"), "20th");
        assert_eq!(what_century("3210"), "33rd");
    }
}

fn what_century(year: &str) -> String {
    let mut y: i32 = year.parse().unwrap();
    let rem = y % 100;
    y /= 100;
    if rem != 0 {
        y += 1;
    }
    let mut result = String::new();
    result.push_str(&y.to_string());
    match y {
        11 | 12 | 13 => result.push_str("th"),
        _ if y % 10 == 1 => result.push_str("st"),
        _ if y % 10 == 2 => result.push_str("nd"),
        _ if y % 10 == 3 => result.push_str("rd"),
        _ => result.push_str("th"),
    }
    result
}

pub fn run() {
    what_century("1119");
    what_century("2000");
    what_century("9900");
    what_century("5699");
    what_century("1450");
    what_century("2180");
    what_century("2006");
    what_century("1203");
}

/* CODEWARS GOOD SOLUTIONS

fn what_century(year: &str) -> String {
    let century = (year.parse::<u32>().unwrap() + 99) / 100;
    let suffix = if century < 20 {
        "th"
    } else {
        match century % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };

    format!("{}{}", century, suffix)
}

*/
