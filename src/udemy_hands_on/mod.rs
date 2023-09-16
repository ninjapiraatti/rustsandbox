pub mod udemy_ho_linkedlists;
pub mod udemy_ho_looping;
pub mod udemy_ho_mergesort;
pub mod udemy_ho_mutability;
pub mod udemy_ho_own_memory;
pub mod udemy_ho_pointers_memory;
pub mod udemy_ho_quicksort;
pub mod udemy_ho_results_options;
pub mod udemy_ho_structs_enums;

pub fn get_programs() -> Vec<(&'static str, fn())> {
    vec![
        ("udemy_ho_linkedlists", udemy_ho_linkedlists::run),
        ("udemy_ho_looping", udemy_ho_looping::run),
        ("udemy_ho_mergesort", udemy_ho_mergesort::run),
        ("udemy_ho_mutability", udemy_ho_mutability::run),
        ("udemy_ho_own_memory", udemy_ho_own_memory::run),
        ("udemy_ho_pointers_memory", udemy_ho_pointers_memory::run),
        ("udemy_ho_quicksort", udemy_ho_quicksort::run),
        ("udemy_ho_results_options", udemy_ho_results_options::run),
        ("udemy_ho_structs_enums", udemy_ho_structs_enums::run),
    ]
}

pub fn runall() {
    udemy_ho_looping::run();
    udemy_ho_mergesort::run();
    udemy_ho_mutability::run();
    udemy_ho_own_memory::run();
    udemy_ho_pointers_memory::run();
    udemy_ho_quicksort::run();
    udemy_ho_results_options::run();
    udemy_ho_structs_enums::run();
    udemy_ho_linkedlists::run();
}
