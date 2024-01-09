pub mod codewars_breakcamelcase;
pub mod codewars_casinochips;
pub mod codewars_century;
pub mod codewars_elevator;
pub mod codewars_hungarian;
pub mod codewars_interlaced;
pub mod codewars_mirror;
pub mod codewars_moviecard;
pub mod codewars_nsmallest;
pub mod codewars_removep;
pub mod codewars_sort_binary_by_levels;
pub mod codewars_sortbybit;
pub mod codewars_stones;
//mod codewars_urlshortener;

pub fn get_programs() -> Vec<(&'static str, fn())> {
    vec![
        ("codewars_casinochips", codewars_casinochips::run),
        ("codewars_century", codewars_century::run),
        ("codewars_elevator", codewars_elevator::run),
        ("codewars_hungarian", codewars_hungarian::run),
        ("codewars_interlaced", codewars_interlaced::run),
        ("codewars_mirror", codewars_mirror::run),
        ("codewars_moviecard", codewars_moviecard::run),
        ("codewars_nsmallest", codewars_nsmallest::run),
        ("codewars_removep", codewars_removep::run),
        ("codewars_sortbybit", codewars_sortbybit::run),
        ("codewars_stones", codewars_stones::run),
        ("codewars_breakcamelcase", codewars_breakcamelcase::run),
        (
            "codewars_sort_binary_by_levels",
            codewars_sort_binary_by_levels::run,
        ),
    ]
}

pub fn runall() {
    //codewars_casinochips::run();
    //codewars_century::run();
    //codewars_elevator::run();
    //codewars_hungarian::run();
    codewars_interlaced::run(); // WIP
                                //codewars_mirror::run();
                                //codewars_nsmallest::run();
                                //codewars_removep::run();
                                //codewars_sortbybit::run();
                                //codewars_stones::run();
                                //codewars_moviecard::run();
                                //codewars_urlshortener::run(); //WIP
                                //codewars_breakcamelcase::run();
}
