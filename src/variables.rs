// Variables are immutable bu default
// Rust is a block-scoped language

pub fn run () {
    // Normal variable doesn't have to be typed on declaration.
    let number = 42;
    // Add mut if you want to change it later.
    let mut number2 = 21;
    // I dunno why would you need constants then but ok. Those you have to type.
    const ID: i32 = 001;
    // Multiple vars at once
    let (toon_name, toon_level) = ("Ti Lai", 10); 

    number2 *= 2;
    println!("{} {} {} {} {}", number, number2, ID, toon_name, toon_level);
}