#include <stdio.h>

int part1() {
	int total = 0;
	int current = 0;
	int max = 0;
	char buffer[16];
	while (fgets(buffer, sizeof(buffer), stdin)) {
		if (sscanf(buffer, "%i\n", &current) == EOF) {
			max = total > max? total : max;
			total = 0;
		} else {
			total += current;
		}
	}
	printf("Part 1: %d\n", max);
	return 0;
}

void get_n_largest(int* max, int n, int value) {
	int i = 0;
	if (value > max[0]) {
		while (value > max[++i] & i < n) {
			max[i-1] = max[i];
		}
		max[i-1] = value;
	}
}

int part2() {
	int total = 0;
	int current = 0;
	int max[3] = {0, 0, 0};
	char buffer[16];
	while(fgets(buffer, sizeof(buffer), stdin)) {
		if(sscanf(buffer, "%i\n", &current) == EOF) {
			get_n_largest(max, sizeof(max)/sizeof(max[0]), total);
			total = 0;
		} else {
			total += current;
		}
	}
	printf("Part 2: %d\n", max[0] + max[1] + max[2]);
	return 0;
}

int main() {
	//part1();
	part2();
	return 0;
}
