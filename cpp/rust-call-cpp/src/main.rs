// Neded for rust to know what is is written in C
extern "C" {

    // declares print_num
    fn print_num(num: i32);
}

/// Main
fn main() {
    println!("[rust] start");
	
	// Unsafe needed because we are calling the function from the outside world
    unsafe {
        print_num(50);
    }

	println!("[rust] end");
}