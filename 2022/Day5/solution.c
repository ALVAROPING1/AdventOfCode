#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define COLUMNS 10
#define ROWS 64

void reverseInPlace(const int length, char string[length+1]) {
	const int half_length = length >> 1;
	char tmp;
	for (int i = 0; i < half_length; i++) {
		tmp = string[i];
		string[i] = string[length - 1 - i];
		string[length - 1 - i] = tmp;
	}
}

void reverseCopy(
	const int length,
	char target[length+1],
	const char source[length+1]
) {
	for (int i = 0; i < length; i++) {
		target[i] = source[length - 1 - i];
	}
	target[length] = '\0';
}

int parseCrates(
	int columns,
	const int rows,
	char crates[columns][rows],
	int items[columns]
) {
	int i = 0;

	// Load first line from input
	char buffer[48];
	fgets(buffer, sizeof(buffer), stdin);
	const int buffer_length = strlen(buffer);

	// Calculate amount of columns needed
	const int input_columns = (buffer_length + 1) >> 2;
	columns = input_columns < columns? input_columns : columns;

	// Initialize amount of items in each column to 0
	for (i = 0; i < columns; i++) {items[i] = 0;}

	// Parse input
	char current_item;
	do {
		// For each column item in the current line
		for (i = 0; i < columns; i++) {
			// If the column item is not empty, add it to the current column
			current_item = buffer[(i<<2) + 1];
			if (current_item != ' ') {
				crates[i][items[i]++] = current_item;
				crates[i][items[i]] = '\0';
			}
		}
	// Repeat until the column header row
	} while (fgets(buffer, sizeof(buffer), stdin)[1] != '1');
	
	// The items on each column are in reverse order, reverse them again
	for (i = 0; i < columns; i++) {reverseInPlace(items[i], crates[i]);}
	
	// Consume the empty line after the column header row
	fgets(buffer, sizeof(buffer), stdin);

	return columns;
}

void moveCrates(
	const int columns,
	const int rows,
	char crates[columns][rows],
	int items[columns],
	const bool reverse
) {
	int source, target, amount;
	const char* format = "move %i from %i to %i\n";
	if (reverse) {
		char reversedString[ROWS];
		// For each movement operation
		while (scanf(format, &amount, &source, &target) != EOF) {
			// Get and reverse the items from source
			reverseCopy(
				amount,
				reversedString,
				// Get the last <amount> items
				&crates[source-1][items[source-1]-amount]
			);
			// Append the reversed source items to target
			strncat(
				crates[target-1],
				reversedString,
				rows - items[target-1] - 1
			);
			// Update the amount of items in source and target
			items[target-1] += amount;
			items[source-1] -= amount;
			crates[source-1][items[source-1]] = '\0';
		}
	} else {
		// For each movement operation
		while (scanf(format, &amount, &source, &target) != EOF) {
			// Append the source items to target
			strncat(
				crates[target-1],
				// Get the last <amount> items
				&crates[source-1][items[source-1]-amount],
				rows - items[target-1] - 1
			);
			// Update the amount of items in source and target
			items[target-1] += amount;
			items[source-1] -= amount;
			crates[source-1][items[source-1]] = '\0';
		}
	}
}

void printTopCrate(
	const int columns,
	const int rows,
	const char crates[columns][rows],
	const int items[columns],
	const int part
) {
	char out[COLUMNS];
	for (int i = 0; i < columns; i++) {
		out[i] = crates[i][items[i] - 1];
	}
	out[columns] = '\0';
	printf("Part %d: %s\n", part, out);
}

int part1() {
	char crates[COLUMNS][ROWS];
	int items[COLUMNS];
	const int columns = parseCrates(COLUMNS, ROWS, crates, items);
	moveCrates(columns, ROWS, crates, items, true);
	printTopCrate(columns, ROWS, crates, items, 1);
	return 0;
}

int part2() {
	char crates[COLUMNS][ROWS];
	int items[COLUMNS];
	const int columns = parseCrates(COLUMNS, ROWS, crates, items);
	moveCrates(columns, ROWS, crates, items, false);
	printTopCrate(columns, ROWS, crates, items, 2);
	return 0;
}

int main() {
	//part1();
	part2();
	return 0;
}
