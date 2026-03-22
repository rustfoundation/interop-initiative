#include <stdio.h>
#include <stdint.h>

extern uint32_t* rust_alloc_array(uint32_t length);
extern void      rust_free_array(uint32_t* ptr, uint32_t length);

int main(void) {
    uint32_t length = 5;

    uint32_t* arr = rust_alloc_array(length);
    printf("[C] received array from Rust:\n");

    for (uint32_t i = 0; i < length; i++) {
        printf("  arr[%u] = %u\n", i, arr[i]);
    }

    rust_free_array(arr, length);
    printf("[C] array freed correctly using rust_free_array\n");

    return 0;
}
