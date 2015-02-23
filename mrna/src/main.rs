use std::old_io as io;

fn main() {
	 let protein = io::stdin().read_line().ok().expect("Reading stdin failed!");    
	 let mut result: u64 = 3; // stop!

	 for c in protein.trim().graphemes(true) {
	 	result %= 1000000;
	 	result *= match c {
	 		"A" => 4,
	 		"C" => 2,
	 		"D" => 2,
	 		"E" => 2,
	 		"F" => 2,
	 		"G" => 4,
	 		"H" => 2,
	 		"I" => 3,
	 		"K" => 2,
	 		"L" => 6,
	 		"M" => 1,
	 		"N" => 2,
	 		"P" => 4,
	 		"Q" => 2,
	 		"R" => 6,
	 		"S" => 6,
	 		"T" => 4,
	 		"V" => 4,
	 		"W" => 1,
	 		"Y" => 2,
	 		_ => panic!("Error, unexpected value: {}",c),
	 	};
	 }

	 println!("{}", result);
}

