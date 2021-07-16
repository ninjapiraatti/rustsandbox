// Pretty normal. No parenthesis

pub fn run() {
    let res;
    res = helloworld("moi", 4);
    println!("{}", res);

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("{}", add_nums(2, 3));

    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    println!("from function: {}", example());
}

// Underscore means you don't use that x but is there a better way?
fn helloworld(str1: &str, number: i32) -> i32 {
    for _x in 0..number {
        println!("{}", str1);
    }
    0 // value without semicolon is considered the return value
}

// Multiple returns
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

// Another example
fn example() -> i32 {
    let x = 42;
    // Rust's ternary expression
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        // notice the braces are optional when its just a single return expression
        _ => "is not hotdog",
    };
    println!("identifying food: {}", result);

    let v = {
        // This scope block lets us get a result without polluting function scope
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    // The idiomatic way to return a value in rust from a function at the end
    v + 4
}
