fn main() {
    println!("cargo:rerun-if-changed=rust/bindings.h");
    println!("cargo:rerun-if-changed=src/main.cpp");
}