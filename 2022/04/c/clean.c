#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int readfromuntil(char *output, int from, int max, char until, char *input);

int main(void) {
	FILE *input = fopen("../input.txt", "r");
	if (input == NULL) {
		printf("Issue loading file\n");
		return 1;
	}

	char line[255];
	char asStrings[4][10];
	int asInts[4];

	int total = 0;

	while (fgets(line, 255, input) != NULL) {
		int lineLength = strlen(line);
		int position = 0;

		position += readfromuntil(asStrings[0], position, 10, '-', line) + 1;
		position += readfromuntil(asStrings[1], position, 10, ',', line) + 1;
		position += readfromuntil(asStrings[2], position, 10, '-', line) + 1;
		position += readfromuntil(asStrings[3], position, 10, '\n', line) + 1;

		asInts[0] = atoi(asStrings[0]);
		asInts[1] = atoi(asStrings[1]);
		asInts[2] = atoi(asStrings[2]);
		asInts[3] = atoi(asStrings[3]);

		if (
			(asInts[0] <= asInts[2] && asInts[1] >= asInts[3]) ||
			(asInts[0] >= asInts[2] && asInts[1] <= asInts[3])
		) {
			total++;
		}
	}

	printf("This is the total we get %i\n", total);
}

int readfromuntil(char *output, int from, int max, char until, char *input) {
	int i = from;
	while (true) {
		if (
			input[i] == until
			|| input[i] == '\n'
			|| (i - from) >= max
		) {
			output[i - from] = '\n';
			break;
		}
		
		output[i - from] = input[i];
		i++;
	}

	return (i - from);
}