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

	let fastas = parse_fasta(input);

	for fasta in fastas {
		println!("{:?}", fasta);
	}
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
