#include <stdio.h>
#include <stdlib.h>

/* Declared in the Rust library */
extern char* process_string(const char* input);
extern void  free_string(char* ptr);

int main(void) {
    const char* message = "Hello from C!";
    printf("[C] sending: \"%s\"\n", message);

    /* Call into Rust — Rust allocates the response string */
    char* response = process_string(message);
    printf("[C] received: \"%s\"\n", response);

    /* Must use Rust's free_string — NOT free() — because Rust allocated it */
    free_string(response);
    printf("[C] string freed successfully\n");

    return 0;
}
