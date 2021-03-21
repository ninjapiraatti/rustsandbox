// https://www.codewars.com/kata/5fee4559135609002c1a1841
// 5 kyu

// Complete the function shorten(), which receives a long URL and 
// returns a shortened URL consisting only of lowercase letters 
// (and one dot and one slash).

// Every short URL has to include short.ly/ and must not be longer than 13.

#[cfg(test)]
mod tests {
    use super::UrlShortener;
    use crate::assert_valid_short_url;
    
    #[test]
    fn two_different_urls() {
        let mut url_shortener = UrlShortener::new();
        
        let short_url_1 = url_shortener.shorten("https://www.codewars.com/kata/5ef9ca8b76be6d001d5e1c3e");
        assert_valid_short_url!(&short_url_1);
        
        let short_url_2 = url_shortener.shorten("https://www.codewars.com/kata/5efae11e2d12df00331f91a6");
        assert_valid_short_url!(&short_url_2);
        
        assert_eq!(url_shortener.redirect(&short_url_1), "https://www.codewars.com/kata/5ef9ca8b76be6d001d5e1c3e");
        assert_eq!(url_shortener.redirect(&short_url_2), "https://www.codewars.com/kata/5efae11e2d12df00331f91a6");
    }
    
    #[test]
    fn same_urls() {
        let mut url_shortener = UrlShortener::new();
        
        let short_url_1 = url_shortener.shorten("https://www.codewars.com/kata/5ef9c85dc41b4e000f9a645f");
        assert_valid_short_url!(&short_url_1);
        
        let short_url_2 = url_shortener.shorten("https://www.codewars.com/kata/5ef9c85dc41b4e000f9a645f");
        assert_valid_short_url!(&short_url_2);
        
        assert_eq!(short_url_1, short_url_2, "Should work with the same long URLs");
        assert_eq!(
            url_shortener.redirect(&short_url_1),
            "https://www.codewars.com/kata/5ef9c85dc41b4e000f9a645f",
            "{} should redirect to https://www.codewars.com/kata/5ef9c85dc41b4e000f9a645f",
            &short_url_1,
        );
    }
    
    #[macro_export]
    macro_rules! assert_valid_short_url {
        ($url:expr) => {
            assert!(
                $url.starts_with("short.ly/"),
                "URL format is incorrect: should start with \"short.ly/\", got: {}",
                $url,
            );
            
            assert!(
                $url.len() < 14,
                "URL format is incorrect: length should be < 14 characters, got: {}",
                $url,
            );
            
            // As the URL contains "short.ly/", we can safely index using [9..]
            assert!(
                $url[9..].bytes().all(|b| b.is_ascii_lowercase()),
                "URL format is incorrect: should contain lowercase letters at the end, got: {}",
                $url,
            );
        }
    }    
}


struct UrlShortener {

}

impl UrlShortener {
    fn new() -> Self {
        Self { }
    }
    
    fn shorten(&self, long_url: &str) -> String {
        todo!()
    }
    
    fn redirect(&self, short_url: &str) -> String {
        todo!()
    }
}

pub fn run () {
    let mut url_shortener = UrlShortener::new();
        
    let short_url_1 = url_shortener.shorten("https://www.codewars.com/kata/5ef9ca8b76be6d001d5e1c3e");
    println!("{:?}", &short_url_1);
}

/* CODEWARS SOLUTIONS

use itertools::Itertools;

fn first_n_smallest(arr: &[i32], n: usize) -> Vec<i32> {
    arr.iter()
        .enumerate()
        .sorted_by_key(|(_, &c)| c)
        .take(n)        
        .sorted_by_key(|(i, _)| *i)
        .map(|(_, &c)| c)
        .collect()
}

*/