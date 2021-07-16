// collect() can take anything iterable, and turn it into a relevant collection.
// This is one of the more powerful methods in the standard library, used in a variety of contexts.

// Using Collect to make a string
pub fn run() {
    let chars = ['g', 'd', 'k', 'k', 'n'];
    let hello: String = chars
        .iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect();
    println!("{}", hello)
}
