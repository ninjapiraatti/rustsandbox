// https://www.codewars.com/kata/562f91ff6a8b77dfe900006e
// 7 kyu

// My friend John likes to go to the cinema. He can choose between system A and system B.

// System A : he buys a ticket (15 dollars) every time
// System B : he buys a card (500 dollars) and a first ticket for 0.90 times the ticket price, 
// then for each additional ticket he pays 0.90 times the price paid for the previous ticket.
// Example:
// If John goes to the cinema 3 times:

// System A : 15 * 3 = 45
// System B : 500 + 15 * 0.90 + (15 * 0.90) * 0.90 + (15 * 0.90 * 0.90) * 0.90 ( = 536.5849999999999, no rounding for each ticket)
// John wants to know how many times he must go to the cinema so that the final result of System B, when rounded up to the next dollar, will be cheaper than System A.

// The function movie has 3 parameters: card (price of the card), ticket (normal price of a ticket), perc (fraction of what he paid for the previous ticket) and returns the first n such that
	#[cfg(test)]
	mod tests {
	use super::*;

	fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
		println!(" card: {:?};", card);
		println!("ticket: {:?};", ticket);
		println!("perc: {:?};", perc);
		let ans = movie(card, ticket, perc);
		println!("actual:\n{:?};", ans);
		println!("expect:\n{:?};", exp);
		println!(" {};", ans == exp);
		assert_eq!(ans, exp);
		println!("{};", "-");
	}

	#[test]
	fn basic_tests() {
		dotest(500, 15, 0.9, 43);
		dotest(100, 10, 0.95, 24);
		dotest(0, 10, 0.95, 2);
		
	}
}

fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
	let mut i = 0.0;
	let mut total_price = card as f64;
	let mut variable_price = ticket as f64;
	loop {
		i += 1.0;
		variable_price *= perc;
		total_price += variable_price;
		if total_price.ceil() < i * ticket as f64 {
			return i as i32;
		}
	}	
}

pub fn run () {
	movie(5, 5, 5.0,);
}

/* CODEWARS SOLUTIONS

fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    let mut sum = card as f64;
    let ticket = ticket as f64;
    for n in 1..{
        sum += ticket * perc.powf(n as f64);
        if sum.ceil() < ticket * n as f64{return n;}
    } 0
}

*/