pub fn run() {
    println!("Hi from print.");
    println!("This is rust's way to printf. Int: {} and string: {}", 42, "Kylpynalle");
    println!("Positional formatting: {0} {1} {0}", "Zero", "One");
    println!("Named arguments: {first} {second}", first = "Turvanuudele", second = "Suurpone");
    println!("And some proper formatting. Here's 42 as binary, hex and octal: {0:b} {0:x} {0:o}", 42);
    println!("Ooh and there's a debug trait: {:?}", (42, "hello", true));
    println!("Basic math. 30 + 12 is {}", 30 + 12);
}