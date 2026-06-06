#include <stdio.h>
#include <stdlib.h>

// #include <string.h>

int main( int argc, char *argv[] ) {
	char *input_file = argv[1];

	FILE *ifile = fopen(input_file, "r");
	if ( ifile == NULL ) {
		printf("Not able to find: './%s'", input_file);
		return -1;
	}
	
	printf("Reading out './%s'\n\n===\n\n", input_file);

	int c; // c will be treated
	while (( c = fgetc( ifile )) != EOF) printf("%c", c);

	printf("\n===\n\nEnd of file: './%s\n", input_file);
	fclose(ifile);
	return 0;
}
