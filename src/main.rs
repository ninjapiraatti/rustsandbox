mod rng;

mod udemy_hands_on;
mod codewars;
mod basics;

// There are many aninmals like birb, formal chikcen, disco chikcen, treefloof, nope rope, sea flap flap, murder log, wizard cow,
// flopwop, danger zebra, stab rabbit, fart squirrel, blub blub doggo, trouble bubble, aquatic sock puppet, water pistachio,
// cheese boi, noodle bear

// There are also many activities like mlem, blep, boop, bave, derp, 

fn main() {
	let nbr = rng::rng(100);
	println!("{:?}", nbr);
	codewars::runall();
	udemy_hands_on::runall();
	basics::runall();
}