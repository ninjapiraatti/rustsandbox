// https://www.codewars.com/kata/57fd696e26b06857eb0011e7/train/rust
// 7 kyu

// Your goal is to create a function dative() (Dative() in C#)
// which returns the valid form of a valid Hungarian word w in 
// dative case i. e. append the correct suffix nek or nak 
// to the word w based on vowel harmony rules.

#[cfg(test)]
mod tests {
    use super::dative;

    #[test]
    fn sample_tests() {
        const TESTS: &[[&str; 2]; 11] = &[
            // [input, expected]
            ["ablak", "ablaknak"],
            ["tükör", "tükörnek"],
            ["keret", "keretnek"],
            ["otthon", "otthonnak"],
            ["virág", "virágnak"],
            ["tett", "tettnek"],
            ["rokkant", "rokkantnak"],
            ["rossz", "rossznak"],
            ["gonosz", "gonosznak"],
            ["rög", "rögnek"],
            ["király", "királynak"]
        ];
        
        for test in TESTS {
            let input = test[0];
            let expected = test[1];
            println!("{} -> {}", input, expected);
            assert_eq!(dative(input), expected);
        }
    }
}


fn dative(word: &str) -> String {
    let mut res = String::from(word);
    for c in word.chars().rev() { 
        match c {
            'a' | 'á' | 'o' | 'ó' | 'u' | 'ú' => {
                res.push_str("nak");
                break;
            }
            'e' | 'é' | 'i' | 'í' | 'ö' | 'ő' | 'ü' | 'ű' => {
                res.push_str("nek");
                break;
            }
            _ => {

            }
        }
    }
    res
}

pub fn run () {
    dative("gonosz");
}

/* CODEWARS MOST UPVOTED

fn dative(word: &str) -> String {
    word.to_owned() + if word.contains(&['a', 'á', 'o', 'ó', 'u', 'ú'][..]) { "nak" } else { "nek" }
}


fn dative(word: &str) -> String {
    word.chars()
        .rev()
        .find_map(|c| match c {
            'e' | 'é' | 'i' | 'í' | 'ö' | 'ő' | 'ü' | 'ű' => Some("nek"),
            'a' | 'á' | 'o' | 'ó' | 'u' | 'ú' => Some("nak"),
            _ => None,
        })
        .map_or(word.to_string(), |suffix| word.to_string() + suffix)
}

*/