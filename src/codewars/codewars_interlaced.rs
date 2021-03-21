// https://www.codewars.com/kata/5a24a35a837545ab04001614/train/rust
// 5 kyu

// In this kata, your task is to implement what I call Interlaced Spiral Cipher (ISC).

// Encoding a string using ISC is achieved with the following steps:

// 1.Form a square large enough to fit all the string characters
// 2.Starting with the top-left corner, place string characters in the corner cells 
// moving in a clockwise direction
// 3.After the first cycle is complete, continue placing characters in the cells following t
// he last one in its respective row/column
// 4.When the outer cells are filled, repeat steps 2 through 4 for the remaining inner 
// squares (refer to the example below for further clarification)
// 5.Fill up any unused cells with a space character and return the rows joined together.

#[cfg(test)]
mod example_tests {
    use super::*;
    
    fn run_test(s1:&str, s2:&str){
        assert_eq!(&isc::encode(s1),s2);
        assert_eq!(&isc::decode(s2),s1);
    }
    
    #[test]
    fn example_test_1() {
        let example_1a = "Romani ite domum";
        let example_1b = "Rntodomiimuea  m";
        run_test(example_1a,example_1b);
    }
    
    #[test]
    fn example_test_2() {
        let example_2a = "Sic transit gloria mundi";
        let example_2b = "Stsgiriuar i ninmd l otac";
        run_test(example_2a,example_2b);
    }
}

mod isc {
    pub fn encode(s: &str) -> String {
        let res = String::from(s);
        println!("{:?}", res.len());
        res
    }
    
    pub fn decode(s: &str) -> String {
        let res = String::from(s);
        println!("{:?}", res);
        res
    }
}

pub fn run () {
    isc::encode("Lorem ipsum dolor sit amet");
}

/* CODEWARS GOOD SOLUTIONS



*/