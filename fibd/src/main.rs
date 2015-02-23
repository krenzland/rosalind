use std::old_io as io;
use std::collections::HashMap;

fn main() {
   // Read numbers
   let (n,m) = get_nums();

   // Fill cache up to n:
   let mut fibs = HashMap::with_capacity(n as usize);
   {
   	let mut fib: Fibonacci = Fibonacci{ curr: 1, next:0};
   	for (i, num) in fib.take((n + 1) as usize).enumerate() {
   		fibs.insert(i,num);
   	}
   }

   let mut result = fib(n+1-m,m, &fibs);

   println!("{}", result);

}

fn fib(n:u64, m: u64, cache: &HashMap<usize,u64>) -> u64 {
	match cache.get(&(n as usize)) {
		Some(&num) => num,
		None => match n {
			0 => 0,
			1 => 1,
			_ => fib(n - 1, m, &cache) + fib(n - 2,m,&cache)
		}
	}
}

struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn get_nums() -> (u64,u64) {
	let input = io::stdin().read_line().ok().expect("Reading stdin failed!");
	let inputs: Vec<&str>= input.split_str(" ").take(2).map(|x| x.trim()).collect();

	if inputs.len() != 2 { panic!("More ints please!") };
	let a: Result<u64,_> = inputs[0].parse();
	let b: Result<u64,_> = inputs[1].parse();
	match (a,b) {
		(Ok(a_), Ok(b_)) => (a_, b_),
		_ => panic!("Not a number.")
	}
}