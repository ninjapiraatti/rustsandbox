// Fixed lists where items are the same data type. Very much like in C.

pub fn run() {
    let numbers: [i32; 5] = [1, 4, 55, 42, 0];

    println!("{:?}", numbers);

    // Arrays are stack allocated
    println!(
        "This array takes {} bytes of memory",
        std::mem::size_of_val(&numbers)
    );

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("{:?}", slice);
}
