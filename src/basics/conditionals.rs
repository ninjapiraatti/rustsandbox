// Pretty normal. No parenthesis

pub fn run() {
    let kylpynalle = 20;

    // Shorthand if
    let many_kylpynalles = if kylpynalle > 3 { true } else { false };
    println!("{}", many_kylpynalles);

    // For range
    for x in 0..100 {
        print!(".{}", x);
    }
}
