use std::old_io as io;

fn main () {

	let input = io::stdin().read_line().ok().expect("");

	let input_num: Result<u32, _> = input.trim().parse();
	let num: u32 = input_num.ok().expect("Well...");

	permutations(num);
}

fn permutations(n: u32) {
	let numbers: Vec<u32> = (1..n+1).collect();
	let count = numbers.permutations().count();
	let permutations = numbers.permutations();
	println!("{:?}", count);
	for n in permutations {
		for n in n {
			print!("{:?} ", n);
		}
		println!("");
	}
}