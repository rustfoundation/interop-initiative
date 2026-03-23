#include<stdio.h>

extern void swap_string(char **a, char **b);

int main() {
	char *x = "Hello";
	char *y = "World";

	printf("Before swap: %s %s\n", x, y);
	swap_string(&x, &y);
	printf("After swap: %s %s\n", x, y);

	return 0;
}
