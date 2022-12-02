#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int read_line (char *buffer) {
	int c = getc(stdin);
	int i = 0;
	while ( c != '\n' && c != EOF ) {
		buffer[i++] = c;
		c = getc(stdin);
	}
	buffer[i] = 0;
	return (c != EOF);
}

int part1() {
	char buffer[32];
	int max = 0;
	int current = 0;
	while(read_line(buffer)) {
		if(strlen(buffer) == 0) {
			max = current > max? current : max;
			current = 0;
		} else {
			current = current + atoi(buffer);
		}
	}
	printf("Part 1: %d\n", max);
	return 0;
}

int part2() {
	char buffer[32];
	int max[3] = {0, 0, 0};
	int current = 0;
	while(read_line(buffer)) {
		if(strlen(buffer) == 0) {
			if(current > max[0]) {
				if(current > max[1]) {
					max[0] = max[1];
					if(current > max[2]) {
						max[1] = max[2];
						max[2] = current;
					} else {
						max[1] = current;
					}
				} else {max[0] = current;}
			}
			current = 0;
		} else {
			current = current + atoi(buffer);
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
