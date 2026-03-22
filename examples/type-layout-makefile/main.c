#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

/* Must match the Rust struct layout exactly */
typedef struct {
    uint8_t  byte;
    uint32_t integer;
    bool     flag;
} Example;

/* Declared in the Rust library */
extern Example create_example(void);

int main(void) {
    Example ex = create_example();
    printf("byte:    %u\n",  ex.byte);
    printf("integer: %u\n",  ex.integer);
    printf("flag:    %s\n",  ex.flag ? "true" : "false");
    return 0;
}
