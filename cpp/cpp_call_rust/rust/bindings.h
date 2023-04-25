// This is the bindings file, it is used to define the interface between C++ and Rust.

// The extern "C" is used to prevent C++ from mangling the function names.
// The addone_rs function is defined here, and it is declared in rust as well.
// Remember, keep this file up to date with the Rust file.
#ifdef __cplusplus
extern "C" {
#endif

// This is the addone_rs function, it takes an integer and returns an integer.
// Nothing complex here, just a simple function.
int addone_rs(int number);

#ifdef __cplusplus
}
#endif