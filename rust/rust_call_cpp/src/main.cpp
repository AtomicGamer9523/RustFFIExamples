// Import to log to console
#include <stdio.h>
#include <cstdlib>
#include <cinttypes>


// Externs Function to C ABI
extern "C" void print_num(std::int32_t num) {
	// C++ alternative to rust's "println!"
	printf("[cpp ] num is %d\n", num);
}
