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
	let mut result = String::new();
	let schars = s.chars();
    for c in schars { 
		if c.is_ascii_uppercase() {
			result.push(' ');
			result.push(c);
		} else {
			result.push(c);
		}
	}
	result
}

pub fn run() {
    break_camelcase("camelCasing");
}

/* CODEWARS SOLUTIONS

fn solution(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c.is_uppercase() {
            res.push(' ');
        }
        res.push(c);
    }
    res
}

fn solution(s: &str) -> String {
    s.chars()
        .flat_map(|c| if c.is_ascii_uppercase() { vec![ ' ', c ] } else { vec![ c ] })
        .collect()
}

*/
