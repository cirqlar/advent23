#include <stdio.h>
#include <stdbool.h>

int main(void)
{
	FILE *fp = fopen("../input.txt", "r");
	if (fp == NULL)
	{
		printf("Couldn't open file\n");
		return 1;
	}
	int score = 0;
	
	while (true) {
		char other = fgetc(fp);
		while (other == '\n')
		{
			other = fgetc(fp);
		}
		if (other == EOF)
		{
			break;
		}

		fgetc(fp); // move past space
		char ours = fgetc(fp);

		switch (ours)
		{
		case 'X':
			score += 1;
			break;
		case 'Y':
			score += 2;
			break;
		case 'Z':
			score += 3;
			break;
		
		default:
			break;
		}

		if (
			(ours == 'X' && other == 'A')
			|| (ours == 'Y' && other == 'B')
			|| (ours == 'Z' && other == 'C')
		) {
			score += 3;
		}
		else if (
			(ours == 'X' && other == 'C')
			|| (ours == 'Y' && other == 'A')
			|| (ours == 'Z' && other == 'B')
		) {
			score += 6;
		}

	}

	printf("Our score is %i\n", score);
}