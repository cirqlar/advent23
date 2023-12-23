#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void insertMax(int *max, int maxLength, int current)
{
	for (int i = 0; i < maxLength; i++)
	{
		if (current > max[i]) {
			int temp = max[i];
			max[i] = current;
			current = temp;
		}
	}
}

int main(void)
{
	FILE *input = fopen("../input.txt", "r");
	if (input == NULL)
	{
		printf("Couldn't open file\n");
		return 1;
	}

	int current = 0;
	int max[3] = { 0, 0, 0 };
	char line[100];

	while (fgets(line, 100, input) != NULL)
	{
		if (strcmp(line, "\n") == 0)
		{
			// 3 == length of max
			insertMax(max, 3, current);
			current = 0;
		}
		else
		{
			int calories = atoi(line);
			current += calories;
		}
	}

	// 3 == length of max
	insertMax(max, 3, current);

	printf("The largest is %i\n", max[0] + max[1] + max[2]);
}