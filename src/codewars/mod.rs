pub mod codewars_casinochips;
pub mod codewars_century;
pub mod codewars_elevator;
pub mod codewars_hungarian;
//mod codewars_interlaced;
pub mod codewars_mirror;
pub mod codewars_nsmallest;
pub mod codewars_removep;
pub mod codewars_sortbybit;
pub mod codewars_stones;
//mod codewars_urlshortener;

pub fn runall() {
	codewars_casinochips::run();
	codewars_century::run();
	codewars_elevator::run();
	codewars_hungarian::run();
	//codewars_interlaced::run(); // WIP
	codewars_mirror::run();
	codewars_nsmallest::run();
	codewars_removep::run();
	codewars_sortbybit::run();
	codewars_stones::run();
	//codewars_urlshortener::run(); //WIP
}