#include <stdio.h>
#include <string.h>

long char2pow2(char a) {
	return 1L << (a >= 'a'? a - 'a' : a - 'A' + 26);
}

long str2bitfield(char* string, int start, int end) {
	long out = 0;
	for (start; start < end; start++) {
		out |= char2pow2(string[start]);
	}
	return out;
}

int part1() {
	int total = 0;
	int line_length, line_half_length;
	char buffer[64];
	while (scanf("%s\n", buffer) != EOF) {
		line_length = strlen(buffer);
		line_half_length = line_length >> 1;
		total += ffsl( // Bit Scan Forwards 64bits + 1
			str2bitfield(buffer, 0, line_half_length) &
			str2bitfield(buffer, line_half_length, line_length)
		);
	}
	printf("Part 1: %d\n", total);
	return 0;
}

int part2() {
	int total = 0;
	int i = 0;
	char buffer[64];
	long current = 0xFFFFFFFFFFFFFFFF;
	while (scanf("%s\n", buffer) != EOF) {
		current &= str2bitfield(buffer, 0, strlen(buffer));
		i = (i + 1) % 3;
		if (i == 0) {
			total += ffsl(current);
			current = 0xFFFFFFFFFFFFFFFF;
		}
		
	}
	printf("Part 2: %d\n", total);
	return 0;
}

int main() {
	//part1();
	part2();
	return 0;
}
