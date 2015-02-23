use std::old_io as io;

fn main() {
   
   println!("{}", fib(6,3));
}


fn fib(n: i64, m: i64) -> i64 {
	match n {
		1 => 1,
		2 => 1,
		_ => if m < n {
				fib(n-1, m) + fib(n-2, 0)  - fib(n-m,m)
			} else {
				fib(n-1, m) + fib(n-2, m)},
	}
}