#![feature(old_io)]
use std::old_io as io;

fn main() {
   
   println!("{}", fib(40,3));
}


fn fib(n: i64, k: i64) -> i64 {
	match n {
		1 => 1,
		2 => 1,
		_ => fib(n-1, k) + k* fib(n-2, k),
	}
}