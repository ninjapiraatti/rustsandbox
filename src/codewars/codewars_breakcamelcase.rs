// https://www.codewars.com/kata/5208f99aee097e6552000148
// 6 kyu

// Complete the solution so that the function will break up camel casing, using a space between words.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(break_camelcase("camelCasing"), "camel Casing");
        assert_eq!(break_camelcase("camelCasingTest"), "camel Casing Test");
    }
}

fn break_camelcase(s: &str) -> String {
	let mut str = String::from(s);
    for mut c in str.chars() { 
		c.make_ascii_uppercase();
		println!("{:?}", c);
	}
	println!("{:?}", str);
	str
}

pub fn run() {
    break_camelcase("camelCasing");
}

/* CODEWARS SOLUTIONS

*/
