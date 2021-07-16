// https://www.codewars.com/kata/5f7c38eb54307c002a2b8cc8/train/rust
// 6 kyu

// In this kata you are given a string for example: "example(unwanted thing)example"
// Your task is to remove everything inside the parentheses as well as the parentheses themselves.

#[cfg(test)]
mod tests {
    use super::remove_parentheses;

    #[test]
    fn sample_tests() {
        assert_eq!(
            remove_parentheses("example(unwanted thing)example"),
            "exampleexample"
        );
        assert_eq!(
            remove_parentheses("example (unwanted thing) example"),
            "example  example"
        );
        assert_eq!(remove_parentheses("a (bc d)e"), "a e");
        assert_eq!(remove_parentheses("a(b(c))"), "a");
        assert_eq!(
            remove_parentheses("hello example (words(more words) here) something"),
            "hello example  something"
        );
        assert_eq!(
            remove_parentheses("(first group) (second group) (third group)"),
            "  "
        );
    }
}

fn remove_parentheses(s: &str) -> String {
    let mut finished = false;
    let mut res = String::from(s);
    let mut i;
    let mut startt;
    let mut endd;
    let mut found_start;
    let mut found_end;
    while finished == false {
        i = 0;
        startt = 0;
        endd = 0;
        found_start = false;
        found_end = false;
        for c in res.chars() {
            if c == '(' {
                startt = i;
                found_start = true;
                //tempchar = s.chars().nth(i).unwrap();
                //println!("start at: {}", tempchar);
            }
            if c == ')' && found_start == true && found_end == false && i > startt {
                endd = i + 1;
                found_end = true;
                //tempchar = s.chars().nth(i).unwrap();
                //println!("start at: {}", tempchar);
            }
            if found_start == true && found_end == true {
                break;
            }
            i += 1;
        }
        if found_start == false || found_end == false {
            finished = true;
        } else {
            res.replace_range(startt..endd, "");
        }
    }
    res
}

pub fn run() {
    remove_parentheses("|(first group) (second group) (third group)|");
}

/* CODEWARS GOOD SOLUTIONS

fn remove_parentheses(s: &str) -> String {
    let mut d = 0;
    s.chars()
        .filter(|c| match c {
            '(' => { d += 1; false },
            ')' => { d -= 1; false },
            _ => d == 0,
        })
        .collect()
}


fn remove_parentheses(s: &str) -> String {
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ if count == 0 => result.push(c),
            _ => (),
        }
    }

    result
}

*/
