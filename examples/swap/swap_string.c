#include<stdio.h>

extern void swap_pointers(const char **a, const char **b);
extern void swap_strings(char *a, char *b, unsigned int a_size, unsigned int b_size);

int main() {
	const char *x = "Hello";
	const char *y = "World";

	printf("Before swap pointer:\n");
	printf(" Address:\t[%p]\t-> x = %s\n", x, x);
	printf(" Address:\t[%p]\t-> y = %s\n", y, y);

	swap_pointers(&x, &y);

	printf("After swap pointer:\n");
	printf(" Address:\t[%p]\t-> x = %s\n", x, x);
	printf(" Address:\t[%p]\t-> y = %s\n", y, y);

	char s1[64] = "Hello";
	char s2[64] = "Welcome to this World";

	printf("Before swap string:\n");
	printf(" Address:\t[%p]\t-> s1 = %s\n", s1, s1);
	printf(" Address:\t[%p]\t-> s2 = %s\n", s2, s2);

	swap_strings(s1, s2, sizeof(s1), sizeof(s2));

	printf("After swap string:\n");
	printf(" Address:\t[%p]\t-> s1 = %s\n", s1, s1);
	printf(" Address:\t[%p]\t-> s2 = %s\n", s2, s2);

	return 0;
}
