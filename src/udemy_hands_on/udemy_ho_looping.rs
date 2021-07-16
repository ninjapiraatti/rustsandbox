// Hands-On Data Structures and Algorithms in Rust
// Video 5: Looping mechanisms in iterators

// Kind of basic stuff here, video has more in depth part of doing your own loops

pub fn loopidiloop() {
    let mut n = 0;
    while n < 10 {
        n += 1;
        println!("Murdermittens.");
    }
    for i in 0..10 {
        println!("{} doggos.", i);
    }
}

pub fn run() {
    loopidiloop();
}
