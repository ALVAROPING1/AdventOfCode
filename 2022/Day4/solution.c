#include <stdio.h>
#include <stdbool.h>

bool rangeIsSubset(int min1, int max1, int min2, int max2) {
	return 1 > ((min1 - min2) * (max1 - max2));
}

bool rangeOverlap(int min1, int max1, int min2, int max2) {
	return min1 <= max2 && min2 <= max1;
}

int part1() {
	int total = 0;
	int min1, max1, min2, max2;
	while (scanf("%d-%d,%d-%d\n", &min1, &max1, &min2, &max2) != EOF) {
		total += rangeIsSubset(min1, max1, min2, max2);
	}
	printf("Part 1: %d\n", total);
	return 0;
}

int part2() {
	int total = 0;
	int min1, max1, min2, max2;
	while (scanf("%d-%d,%d-%d\n", &min1, &max1, &min2, &max2) != EOF) {
		total += rangeOverlap(min1, max1, min2, max2);
	}
	printf("Part 2: %d\n", total);
	return 0;
}

int main() {
	//part1();
	part2();
	return 0;
}
