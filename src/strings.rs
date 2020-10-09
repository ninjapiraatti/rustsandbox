// Primitive str is immutable, fixed-length string somewhere in memory.
// String is a growable, heap-allocated data structure. Use when you need to modify or own string data.

pub fn run () {
    // str
    let hello = "Hello";

    // String
    let mut bye = String::from("Bye bye!");

    println!("{} and {}", hello, bye);
    println!("Length of bye: {}", bye.len());
    bye.push('\u{1F980}');
    bye.push_str("Crab");
    println!("And here's bye with crab: {}", bye);

    // Contain aka. strstr
    println!("Does crab contain 'rab'? {}", bye.contains("rab"));
    
    // Loop through string by whitespace
    for word in "asb asn thn ksm tgh".split_whitespace() {
        println!("{} ", word);
    }

    // Assertion testing
    assert_eq!(16, bye.len());
}