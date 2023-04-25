#include <stdio.h>
#include "../rust/bindings.h"

int main(int argc, const char** argv) {
    // Call Rust function
    int result = addone(2);
    printf("[c++ ] %d\n",result);

    return 0;
}