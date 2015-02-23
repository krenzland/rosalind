#![feature(hash)]

use std::collections::HashMap;
use std::old_io as io;

// https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() {
	 let monoisotopic_mass_table: HashMap<&'static str,f64> = hashmap![
	 					"A" => 71.03711,"C" => 103.00919,"D" => 115.02694,"E" => 129.04259,"F" => 147.06841,"G" => 57.02146
						,"H" => 137.05891,"I" => 113.08406,"K" => 128.09496,"L" => 113.08406,"M" => 131.04049,"N" => 114.04293,
						"P" => 97.05276,"Q" => 128.05858,"R" => 156.10111,"S" => 87.03203,"T" => 101.04768,"V" => 99.06841
						,"W" => 186.07931,"Y" => 163.06333];

	 let protein = io::stdin().read_line().ok().expect("Reading stdin failed!");

	 let mut result = 0.0;

	 for c in protein.trim().graphemes(true) {
	 	let result = match monoisotopic_mass_table.get(c) {
	 			Some(&f) => result += f,
	 			None => panic!("Unexpected lookup error!"), 
	 		};
	 }

	 println!("{}", result);
}