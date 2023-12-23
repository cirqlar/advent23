#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void)
{
	FILE *input = fopen("../input.txt", "r");
	if (input == NULL)
	{
		printf("Couldn't open file\n");
		return 1;
	}

	int current = 0;
	int max = 0;
	char line[100];

	while (fgets(line, 100, input) != NULL)
	{
		if (strcmp(line, "\n") == 0)
		{
			if (current > max) {
				max = current;
			}

			current = 0;
		}
		else
		{
			int calories = atoi(line);
			current += calories;
		}
	}

	if (current > max) max = current;

	printf("The largest is %i\n", max);
}