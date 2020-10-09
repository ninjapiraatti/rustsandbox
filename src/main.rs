mod print;
mod variables;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod functions;
mod pointers;
mod structs;
mod enums;
mod cli;
mod mmatch;
mod loops;
mod seacreature;

fn main() {
    print::run();
    variables::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    functions::run();
    pointers::run();
    structs::run();
    enums::run();
    cli::run();
    mmatch::run();
    loops::run();
    seacreature::run();
}