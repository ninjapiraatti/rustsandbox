pub mod arrays;
pub mod boxx;
pub mod cli;
pub mod collectioon;
pub mod conditionals;
pub mod enums;
pub mod functions;
pub mod generics;
pub mod mmatch;
pub mod oopish;
pub mod ownership;
pub mod pointers;
pub mod print;
pub mod seacreature;
pub mod strings;
pub mod structs;
pub mod tuples;
pub mod types;
pub mod variables;
pub mod vectors;

pub fn get_programs() -> Vec<(&'static str, fn())> {
    vec![
        ("arrays", arrays::run),
        ("boxx", boxx::run),
        ("cli", cli::run),
        ("collectioon", collectioon::run),
        ("conditionals", conditionals::run),
        //("enums", enums::run),
        ("functions", functions::run),
        ("generics", generics::run),
        ("mmatch", mmatch::run),
        ("oopish", oopish::run),
        ("ownership", ownership::run),
        ("pointers", pointers::run),
        ("print", print::run),
        ("seacreature", seacreature::run),
        ("strings", strings::run),
        ("structs", structs::run),
        ("tuples", tuples::run),
        ("types", types::run),
        ("variables", variables::run),
        ("vectors", vectors::run),
    ]
}

pub fn runall() {
    arrays::run();
    boxx::run();
    cli::run();
    collectioon::run();
    conditionals::run();
    enums::run().ok();
    functions::run();
    generics::run();
    mmatch::run();
    oopish::run();
    ownership::run();
    pointers::run();
    print::run();
    seacreature::run();
    strings::run();
    structs::run();
    tuples::run();
    types::run();
    variables::run();
    vectors::run();
}
