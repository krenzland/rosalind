#![feature(old_io)]
#![feature(unicode)] 

use std::old_io as io;


fn main () {

let input: String = io::stdin().read_line().ok().expect("Fuck.");

let mut count_a = 0;
let mut count_c = 0;
let mut count_g = 0;
let mut count_t = 0;


for i in input.graphemes(true) {
	match i {
		"A" => count_a += 1,
		"C" => count_c += 1,
		"G" => count_g += 1,
		"T" => count_t += 1,
		_ => (),
	}
}

print!("{} {} {} {}", count_a, count_c, count_g, count_t);

}