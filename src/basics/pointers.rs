// Refernce pointers point to a resource in memory

pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));

    // With non-primitives if you assign another variable to a piece of data, the first variable will
    // no longer hold that value. Use & to reference the value.
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("{:?}", (&vec1, vec2));

    // But this works regardless
    let vec3 = vec![1, 2, 3];
    println!("{:?}", (vec3));
}
