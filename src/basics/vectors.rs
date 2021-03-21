// Vectors are resizable arrays

pub fn run () {
    let mut numbers: Vec<i32> = vec![1, 4, 55, 42, 0];
    println!("{:?}", numbers);

    // Add to vector
    numbers.push(66);
    println!("{:?}", numbers);

    // Vectors are stack allocated
    println!("This array takes {} bytes of memory", std::mem::size_of_val(&numbers));

    // Pop off last value
    numbers.pop();

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("{} {}", numbers[1], slice[0]);

    // Loop through vector values
    for x in numbers.iter() {
        println!("{}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *=4;
        println!("{}", x);
    }
}