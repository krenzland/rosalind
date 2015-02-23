use std::old_io as io;

fn main () {

	let s = io::stdin().read_line().ok().expect("");
	let t = io::stdin().read_line().ok().expect("");

	let mut distance = 0;

	let iterator = s.graphemes(true).zip(t.graphemes(true));

	for (s,t) in iterator {
		if s != t {
			distance += 1;
		}
	}
	
	println!("{}", distance);	
	
}