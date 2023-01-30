#include <stdio.h>

int solve(const int max_size) {
	int i = 1;
	int window_start = 0;
	int last_seen_index;
	int last_seen[26] = {0};
	char current;
	while ((current = fgetc(stdin)) != EOF) {
		last_seen_index = last_seen[current - 'a'];
		if (last_seen_index > window_start) {window_start = last_seen_index;}
		else if (i - window_start >= max_size) {return i;}
		last_seen[current - 'a'] = i;
		i++;
	}
}

int part1() {
	printf("Part 1: %d\n", solve(4));
	return 0;
}

int part2() {
	printf("Part 2: %d\n", solve(14));
	return 0;
}

int main() {
	//part1();
	part2();
	return 0;
}
