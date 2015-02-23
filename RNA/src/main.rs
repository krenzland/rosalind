use std::old_io as io;

fn main() {
   
   let dna = io::stdin().read_line().ok().expect("Fuck.");

   let rna: String = dna.as_slice().replace("T", "U");

   println!("{}", rna);
}

fn replace(dna: &str) -> &str {
	match dna {
		"T" => "U",
		dna => dna
	}

}