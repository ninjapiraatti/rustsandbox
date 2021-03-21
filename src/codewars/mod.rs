mod codewars_casinochips;
mod codewars_century;
mod codewars_elevator;
mod codewars_hungarian;
//mod codewars_interlaced;
mod codewars_mirror;
mod codewars_nsmallest;
mod codewars_removep;
mod codewars_sortbybit;
mod codewars_stones;
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