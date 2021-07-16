// Tuples group together values (of different type if need be)
// Max 12 elements wtf

pub fn run() {
    let person: (&str, &str, i8) = ("Ti", "Lai", 10);

    println!(
        "{} is from {} and is level {}",
        person.0, person.1, person.2
    );
}
