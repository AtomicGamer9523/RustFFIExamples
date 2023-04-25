// This is the bindings file, it is used to define the interface between C++ and Rust.

// The extern "C" is used to prevent C++ from mangling the function names.
// The addone_rs function is defined here, and it is declared in rust as well.
// Remember, keep this file up to date with the Rust file.
#pragma once
#ifdef __cplusplus
extern "C" {
#endif
    /**
     * @brief add one to a number
     * 
     * @param number number to add one to
     * @return int the result of adding one to the number
     */
    int addone_rs(int number);

#ifdef __cplusplus
}
#endif