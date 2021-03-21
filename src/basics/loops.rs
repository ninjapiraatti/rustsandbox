pub fn run() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);
}

/*
    for (i, c) in my_str.chars().enumerate() {
        // do something with character `c` and index `i`
    }
    */