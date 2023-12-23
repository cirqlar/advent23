#include <stdbool.h>
#include <stdio.h>
#include <string.h>

int main(void) {
	FILE *input = fopen("../input.txt", "r");
	if (input == NULL) {
		printf("Issue opening file\n");
		return 1;
	}

	char text[5000];
	if (fgets(text, 5000, input) == NULL) {
		printf("Issue reading line\n");
		return 1;
	}

	int length = strlen(text);
	for (int i = 0; i < length - 3; i++) {
		bool testfail = false;

		for (int j = i; j < i + 3; j++) {
			for (int k = j+1; k <= i + 3; k++) {
				if (text[j] == text[k]) {
					testfail = true;
					break;
				}
			}

			if (testfail) break;
		}

		if (!testfail) {
			printf("We get a this %i\n", i + 4);
			break;
		}
	}
}