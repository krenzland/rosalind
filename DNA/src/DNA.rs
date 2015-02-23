use std::old_io as io;


fn main () {

let input: String = io::stdin().read_line().ok().expect("Fuck.");

let mut countA = 0;
let mut countC = 0;
let mut countG = 0;
let mut countT = 0;


for i in input.graphemes(true) {
	match i {
		"A" => countA += 1,
		"C" => countC += 1,
		"G" => countG += 1,
		"T" => countT += 1,
		_ => (),
	}
}

print!("{} {} {} {}", countA, countC, countG, countT);

}