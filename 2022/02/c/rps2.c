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
		char todo = fgetc(fp);
		char ours;

		if (todo == 'X')
		{
			if (other == 'A') ours = 'Z';
			if (other == 'B') ours = 'X';
			if (other == 'C') ours = 'Y';
		}
		if (todo == 'Y')
		{
			score += 3;
			if (other == 'A') ours = 'X';
			if (other == 'B') ours = 'Y';
			if (other == 'C') ours = 'Z';
		}
		if (todo == 'Z')
		{
			score += 6;
			if (other == 'A') ours = 'Y';
			if (other == 'B') ours = 'Z';
			if (other == 'C') ours = 'X';
		}

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
	}

	printf("Our score is %i\n", score);
}