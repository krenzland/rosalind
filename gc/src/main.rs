// Given: At most 10 DNA strings in FASTA format (of length at most 1 kbp each).

// Return: The ID of the string having the highest GC-content, followed by the GC-content of that string.
// Rosalind allows for a default error of 0.001 in all decimal answers
// unless otherwise stated; please see the note on absolute error below.

#![feature(io)]
#![feature(path)]
#![feature(core)]
#![feature(env)]

use std::env;
use std::old_io as io;
use std::old_io::File;

#[derive(Debug)]
struct Fasta {
	label: String,
	content: String,
}

fn main () {
  	let args: Vec<String> = env::args().map(|x| x.to_string())
                                       .collect();

    let path = match args.as_slice() {
        [_, ref path] => Path::new(path),
        _ => Path::new("rosalind_gc.txt"),

    };

	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", path.display(), why.desc),
		Ok(file) => file,
	};

	let content = match file.read_to_string() {
		Err(why) => panic!("couldn't read {}: {}", path.display(), why.desc),
		Ok(string) => string, 
	};

	let input: &str = content.as_slice();

	let dnas = parse_fasta(input);

	let mut max_name = "".to_string();
	let mut max_gc_content = 0.0;

	for dna in dnas {
		let gc_content = get_gc_content(&dna);
		if (max_gc_content <= gc_content) {
			max_name = dna.label;
			max_gc_content = gc_content;
		}
	}

	println!("{}\n{}", max_name, max_gc_content);

}

fn get_gc_content(dna: &Fasta) -> f64 {
	let mut counter = 0;
	let size = dna.content.len();

	for c in dna.content.graphemes(true) {
		if c == "C" || c == "G" {
			counter += 1;
		}
	}
	(100 * counter) as f64/size as f64
}

fn parse_fasta(input: &str) -> Vec<Fasta> {
	let mut result: Vec<Fasta> = Vec::new();
	let codes: Vec<&str> = input.split('>').collect();

	for c in codes {
		let tokens: Vec<&str> = c.splitn(1, '\n').collect();
		if tokens.len() == 2 {
			let res = vec_to_fasta(tokens);
			result.push(res);
		}
	}

	result
}


fn vec_to_fasta(vec: Vec<&str>) -> Fasta {
	let label = vec[0].to_string();
	let content = vec[1].replace("\n","");
	Fasta {label: label, 
			content: content}
	}

