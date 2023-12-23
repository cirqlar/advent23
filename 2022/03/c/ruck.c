#include <ctype.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>

int main(void)
{
	FILE *input = fopen("../input.txt", "r");
	if (input == NULL)
	{
		printf("Failed to open file\n");
		return 1;
	}
	int total = 0;	
	
	char line[255];
	while (fgets(line, 255, input) != NULL)
	{
		int lineLength = strlen(line);
		int mid = lineLength/2;

		int currentPosition = mid;
		char currentChar = line[currentPosition];
		while (currentChar != '\n')
		{
			bool found = false;

			for (int i = 0; i < mid; i++)
			{
				if (line[i] == currentChar)
				{
					// Add score
					total += (int)currentChar - (isupper(currentChar) ? 38: 96);
					found = true;
					break;
				}
			}

			if (found) break;
			currentPosition++;
			currentChar = line[currentPosition];
		}
	}

	printf("The total %i\n", total);
}