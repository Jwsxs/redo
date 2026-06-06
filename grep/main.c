#include <stdio.h>
#include <string.h>

int main( int argv, char *argc[] ) {
	char *i_file = argc[1];
	FILE *file = fopen(i_file, "r");
	
	if ( file == NULL ) {
		printf("Couldn't find file './%s'", i_file);
		return -1;
	}
	
	char *find_str = argc[2];
	printf("Searching for: \033[1m%s\033[0m at './%s'\n\n===\n\n", find_str, i_file);

	int c;
	int i = 0;
	int str_size = strlen(find_str);
	
	while ((c = fgetc(file)) != EOF) {
		if (find_str[i] == c) {
			i++;
			if ( str_size == i ) {
				printf("\033[1m\033[91m");
				for (int j = 0; j < str_size; j++)
					putchar(find_str[j]);
				i = 0;
			}
		} else {
			printf("\033[0m");
			for (int j = 0; j < i; j++)
				putchar(find_str[j]);
			putchar(c);
			i = 0;
		}
	}
	
	printf("\033[0m\n"); // reset
	fclose(file);

	return 0;
}
