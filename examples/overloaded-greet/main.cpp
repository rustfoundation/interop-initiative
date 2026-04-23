#include <stdio.h>

// here we define 2 functions for the overload -> for greeting
// same name different inputs 
void greet(int x){
    printf("Number: %d\n",x);
}
void greet(const char *name){
    printf("Hello: %s\n",name);
}

// as rust only understand c - style funcs.
// so i created wrapper functions  with diff. names using extern c
extern "C"{
    // Wrap for the number func inp->int
    void greet_number(int x){
        greet(x);
    }
    // wrap for the name func taking the char inp
    void greet_name(const char *name){
        greet(name);
    }
}