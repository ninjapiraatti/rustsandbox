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
	let mut result = String::with_capacity(s.len());
	let mut schars = s.chars();
    for c in schars { 
		result.push(c.to_ascii_uppercase());
		//println!("{:?}", c);
	}
	println!("{:?}", result);
	result
}

pub fn run() {
    break_camelcase("camelCasing");
}

/* CODEWARS SOLUTIONS

*/
