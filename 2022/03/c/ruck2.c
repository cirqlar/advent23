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
	
	char line1[255];
	char line2[255];
	char line3[255];

	while (fgets(line1, 255, input) != NULL)
	{
		fgets(line2, 255, input);
		fgets(line3, 255, input);
		if (line2 == NULL || line3 == NULL)
		{
			printf("Massive fail\n");
			return 1;
		}

		int line1Length = strlen(line1);
		int line2Length = strlen(line2);
		int line3length = strlen(line3);

		for (int i = 0; i < line1Length; i++)
		{
			bool found = false;

			for (int j = 0; j < line2Length; j++)
			{
				if (line1[i] == line2[j])
				{
					for (int k = 0; k < line3length; k++)
					{
						if (line1[i] == line3[k])
						{
							// Add score
							total += (int)line1[i] - (isupper(line1[i]) ? 38: 96);
							found = true;
							break;
						}
					}
				}
				if (found) break;
			}

			if (found) break;
		}
	}

	printf("The total %i\n", total);
}