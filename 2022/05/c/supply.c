#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct item {
	char crateletter;
	struct item *below;
} item;

void printstack(item *stacks[], int numofstacks);
int readuntilline(char *output, int maxlen, int maxlines, char *until, FILE *input);
int readfromuntil(char *output, int from, int max, char until, char *input);

int main(void) {
	FILE *input = fopen("../input.txt", "r");
	if (input == NULL) {
		printf("Issue opening file\n");
		return 1;
	}

	char stacklist[512];
	int numoflines = readuntilline(stacklist, 50, 10, "\n", input);
	int totallength = strlen(stacklist);
	int lenghtofline = totallength/numoflines;
	int numofstacks = stacklist[totallength - 3] - '0';

	item *stacks[12];

	for (int i = numoflines - 2; i >= 0; i--) {
		int offset = i * lenghtofline;
		for (int j = 0; j < numofstacks; j++) {
			char current = stacklist[offset + (4*j) + 1];
			if (isalpha(current)) {
				item *newitem = malloc(sizeof(item));
				if (newitem == NULL) {
					printf("Issue creating item for %c\n", current);
					return 1;
				}

				newitem->crateletter = current;
				newitem->below = stacks[j];
				stacks[j] = newitem;
			}
		}
	}
	
	printstack(stacks, numofstacks);

	char instruction[30];
	while (fgets(instruction, 30, input) != NULL) {
		if (strlen(instruction) < 17) {
			continue;
		}

		char movestr[4];
		char fromstring[5];
		char tostr[4];

		int position = 5;
		position += readfromuntil(movestr, position, 4, ' ', instruction) + 6;
		int frompos = position;
		position += readfromuntil(fromstring, position, 4, ' ', instruction) + 4;
		position += readfromuntil(tostr, position, 4, '\n', instruction);

		printf("Our fromstr %s\n", fromstring);

		int move = atoi(movestr);
		int from = atoi(fromstring) - 1;
		int to = atoi(tostr) - 1;

		// For some reason the last one fails the first time?? I don't know why.
		if (from == -1) {
			readfromuntil(fromstring, frompos, 4, ' ', instruction);
			printf("Our fromstr now %s\n", fromstring);		
			from = atoi(fromstring) - 1; 
		}

		for (int i = 0; i < move; i ++) {
			item *moving = stacks[from];
			if (moving == NULL) {
				printf("Moving something that isn't there \n%s", instruction);
				printstack(stacks, numofstacks);
				return 1;
			}
			stacks[from] = moving->below;

			moving->below = stacks[to];
			stacks[to] = moving;
		}
	}

	printstack(stacks, numofstacks);

	for (int i = 0; i < numofstacks; i++) {
		item *next = stacks[i];
		while (next != NULL) {
			item *current = next;
			next = current->below;

			free(current);
		}
	}
}

void printstack(item *stacks[], int numofstacks) {
	printf("Printing from the top\n");
	for (int i = 0; i < numofstacks; i++) {
		item *next = stacks[i];
		printf("Stack %i: ", i + 1);
		while (next != NULL) {
			printf("%c, ", next->crateletter);
			next = next->below;
		}
		printf("\n");
	}
}

int readuntilline(char *output, int maxlen, int maxlines, char *until, FILE *input) {
	char line[255];
	int lines = 0;
	while (fgets(line, 255, input) != NULL) {
		if (
			strcmp(line, until) == 0
			|| strlen(line) > maxlen
			|| lines >= maxlines
		) {
			break;
		}
		
		strcat(output, line);
		lines++;
	}

	return lines;
}

int readfromuntil(char *output, int from, int max, char until, char *input) {
	int i = from;
	while (true) {
		if (
			input[i] == until
			|| input[i] == '\n'
			|| (i - from) >= max
		) {
			output[i - from] = '\0';
			break;
		}
		
		output[i - from] = input[i];
		i++;
	}

	return (i - from);
}