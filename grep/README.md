also written by the magician Ken, _grep_ comes from a sequence of keys from `ed` editor:  
by typing `g/re/p` you'd **g**lobally search for a **re**gular expression and **p**rint them out  

## how it works
just like _cat_, but not printing everything, you need a "temp buffer" for checking if the string matches  
```c
// whole lot of opening file
// inside of main while:
if ( input_str[i] == c ) {
    i++; // prepare to get the next char
    // but if it matches with the string size:
    if ( i == strlen(input_str) ) {
        // ANSI code for bright red character
        printf("\033[91m");
        // then you print out each character,
        // by using putchar() you automatically move the cursor
        for ( int j = 0; j < strlen(input_str); j++ ) putchar(input_str[j]);
        // and reset your "buffer" for the next one
        i = 0;
    }
    // but, if it fails, it means there's some intruder
    else {
        // and your logic is the same,
        printf("\033[0m");
        // but changes a bit:
        // instead of printing out every char, you print the ones you've already got
        for ( int j = 0; j < i; j++ ) putchar(find_str[j]);
        
        // and keep on going with the normal cat thing
        putchar(c);
        i = 0;
    }
}
```
