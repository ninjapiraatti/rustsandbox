pub fn run () {
    // Default is i32
    let x = 1;
    // Default is f64
    let y = 4.2;
    // Add explicit type
    let z: i64 = 454545454;
    // Find max size
    println!("Max i32 is: {}", std::i32::MAX);
    // Boolean from expression
    let is_greater = 10 < 5;
    // Single character. Unicode, single quotes. That's a crab!
    let c = '\u{1F980}';
    println!("{:?}", (x, y, z, is_greater, c));
}