
/// Build script, build C++ code and link it to Rust.
fn main() {

    println!("cargo:rerun-if-changed=src/main.cpp");
	
    // Handles all the background compiling and linking.
    cc::Build::new()

	// Set the language to C++
    .cpp(true)

    // Files to compile
    .files([
        "src/main.cpp"
    ])
	
    // Starts Compiling after this
    .compile("rustycpp");
}