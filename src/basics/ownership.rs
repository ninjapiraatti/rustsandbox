struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

pub fn run() {
    // We instantiate structs and bind to variables
    // to create memory resources
    // foo_a and foo_b are owners
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 15 };

    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    // foo_b is dropped here
    // foo_a is dropped here
    // Drop is essentially the same as free in C done automatically. Memory is allocated on instanciation/initialization

    // Borrowing Mutable Ownership with References
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // FAILURE: do_something(foo) would fail because
    // foo cannot be moved while mutably borrowed

    // FAILURE: foo.x = 13; would fail here because
    // foo is not modifiable while mutably borrowed

    f.x = 13;
    // f is dropped here because it's no longer used after this point

    println!("{}", foo.x);

    // this works now because all mutable references were dropped
    foo.x = 7;

    // move foo's ownership to a function
    do_something(foo);

    // Using &mut references, you can set the owner's value using the * operator.
    let mut boo = 21;
    let f = &mut boo;
    let bar = *f; // get a copy of the owner's value
    *f = 28; // set the reference's owner's value
    println!("{}", bar);
    println!("{}", boo);
}
