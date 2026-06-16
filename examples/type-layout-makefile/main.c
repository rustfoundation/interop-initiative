#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <assert.h>

/* Must match the Rust #[repr(C)] struct layout exactly:
 *
 *   pub struct Example {
 *       pub byte:    u8,   -> uint8_t
 *       pub integer: u32,  -> uint32_t
 *       pub flag:    bool, -> _Bool / bool
 *   }
 *
 * Note: Rust's bool and C's _Bool are both guaranteed to be 1 byte and use
 * 0/1 values, but this equivalence is a convention — there is no
 * compiler-enforced check across the FFI boundary.
 */
typedef struct {
    uint8_t  byte;
    uint32_t integer;
    bool     flag;
} Example;

/* Compile-time layout checks: verify field offsets and total size match
 * what Rust's #[repr(C)] produces for the same field order and types. */
static_assert(offsetof(Example, byte)    == 0,  "byte offset mismatch");
static_assert(offsetof(Example, integer) == 4,  "integer offset mismatch");
static_assert(offsetof(Example, flag)    == 8,  "flag offset mismatch");
static_assert(sizeof(Example)            == 12, "Example size mismatch");

/* Declared in the Rust library */
extern Example create_example(void);

int main(void) {
    Example ex = create_example();

    printf("byte:    %u\n",  ex.byte);
    printf("integer: %u\n",  ex.integer);
    printf("flag:    %s\n",  ex.flag ? "true" : "false");

    printf("\nField offsets (bytes):\n");
    printf("  byte:    %zu\n", offsetof(Example, byte));
    printf("  integer: %zu\n", offsetof(Example, integer));
    printf("  flag:    %zu\n", offsetof(Example, flag));
    printf("  sizeof:  %zu\n", sizeof(Example));

    return 0;
}
