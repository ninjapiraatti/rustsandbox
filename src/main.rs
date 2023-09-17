pub mod basics;
pub mod codewars;
pub mod rng;
pub mod udemy_hands_on;

use clap::{arg, Command};
use std::str::{self};

// There are many aninmals like birb, formal chikcen, disco turkey, treefloof, nope rope, sea flap flap, murder log, wizard cow,
// flopwop, danger zebra, stab rabbit, fart squirrel, blub blub doggo, trouble bubble, aquatic sock puppet, water pistachio,
// cheese boi, noodle bear, bunno, murder torpedo, fashion chikcen

// There are also many activities like mlem, blep, boop, bave, derp,


fn main() {
    let program_list = list_programs();
    let matches = Command::new("Rust Snippets")
        .version("1.0")
        .after_help(program_list)
        .about("Runs little Rust experiments")
        .arg(arg!([category_number] "Category as number").required(true))
        .arg(arg!([program_number] "Program as number").required(true))
        .get_matches();
    if let Some(category_number) = matches.get_one::<String>("category_number") {
        if let Some(program_number) = matches.get_one::<String>("program_number") {
            if let Ok(category_idx) = category_number.parse::<usize>() {
                if let Ok(program_idx) = program_number.parse::<usize>() {
                    run_program(category_idx, program_idx);
                } else {
                    eprintln!("Failed to parse program number as a usize");
                }
            } else {
                eprintln!("Failed to parse category number as a usize");
            }
        } else {
            eprintln!("Program number not provided");
        }
    } else {
        eprintln!("Category number not provided");
    }    
}

fn list_programs() -> String {
    let programs = get_categories();
    let program_list: Vec<String> = programs
        .iter()
        .enumerate()
        .map(|(i, program)| format!("{}: {}", i, program.0))
        .collect();
    
    let program_list_str = program_list.join("\n");
    format!("Available programs:\n{}", program_list_str)
}

fn run_program(category_idx: usize, program_idx: usize) {
    let categories = get_categories();
    if let Some((_, get_programs_fn)) = categories.get(category_idx) {
        let programs = get_programs_fn();
        if let Some((_, program_fn)) = programs.get(program_idx) {
            program_fn();
        } else {
            eprintln!("Program index '{}' not found in category index '{}'", program_idx, category_idx);
        }
    } else {
        eprintln!("Category index '{}' not found", category_idx);
    }
}

type ProgramGetter = fn() -> Vec<(&'static str, fn())>;

fn get_categories() -> Vec<(&'static str, ProgramGetter)> {
    vec![
        ("Basics", basics::get_programs),
        ("Codewars Katas", codewars::get_programs),
        ("Udemy Hands On", udemy_hands_on::get_programs),
    ]
}

