#include <stdio.h>

#define B 'X'
#define A 'A'

int mod(int a, int b) {
	int r;
	r = a % b;
	return r < 0? r + b : r;
}

int part1() {
	int total = 0;
	char a, b;
	while (scanf("%c %c\n", &a, &b) != EOF) {
		total += b - B + 1 + mod(b - a + A - B + 1, 3) * 3;
	}
	printf("Part 1: %d\n", total);
	return 0;
}

int part2() {
	int total = 0;
	char a, b;
	while (scanf("%c %c\n", &a, &b) != EOF) {
		total += (b - B) * 3 + 1 + mod(a + b - A - B - 1, 3);
	}
	printf("Part 2: %d\n", total);
	return 0;
}

int main() {
	//part1();
	part2();
	return 0;
}
