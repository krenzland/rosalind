use std::old_io as io;

fn main() {
   
   let dna = io::stdin().read_line().ok().expect("");

   for l in dna.graphemes(true).rev().map(replace) {
   		print!("{}",l);
   }
}

fn replace(s: &str) -> &str {
	match s {
		"A" => "T",
		"T" => "A",
		"C" => "G",
		"G" => "C",
		_ => s,
	}
}