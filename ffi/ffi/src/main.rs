extern crate libc;
use libc::c_int;

extern {
    fn addTwo(num: c_int) -> c_int;
}

fn safe_add_two(num: i32) -> i32 {
	let result = unsafe { addTwo(num) };
	return result;
}

fn main() {
	let x = safe_add_two(0);
	println!("If you add two to zero you get: {}", x);
}