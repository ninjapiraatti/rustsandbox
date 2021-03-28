pub mod rng;
pub mod udemy_hands_on;
pub mod codewars;
pub mod basics;
pub mod ui;

// There are many aninmals like birb, formal chikcen, disco turkey, treefloof, nope rope, sea flap flap, murder log, wizard cow,
// flopwop, danger zebra, stab rabbit, fart squirrel, blub blub doggo, trouble bubble, aquatic sock puppet, water pistachio,
// cheese boi, noodle bear, bunno, murder torpedo, fashion chikcen

// There are also many activities like mlem, blep, boop, bave, derp,

fn main() {

	let nbr = rng::rng(100);
	//codewars::runall();
	//udemy_hands_on::runall();
	//basics::runall();
	//udemy_hands_on::udemy_ho_quicksort::run();
	ui::main(nbr);
}