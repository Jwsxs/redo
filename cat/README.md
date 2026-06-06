_cat_ comes from con**cat**enation, it was written around the 70s by two wizards (Ken and Dennis) for PDP-11's UNIX system  
it's purpose was for concatenating two files together - _try `cat 1.txt 2.txt > 3.txt` and you'll understand it_, but became widely used for just printing out a single one and that's it.

## how it works
_cat_ (_for printing_) does a very simple - _yet useful_ - approach:  
 - reads the file until it reaches `EOF`  

that's it, no more shenanigans

## explanation
unlike what `freads()` says on it's name, we're **not** using it, since we'd need to either have a BUFFER, get it's size and do whoever knows with it  
or even some kinda bullshit like checking binary and converting to ascii then printing out then reading escape codes than... you get it.  

only by opening the file with `fopen()`, we have already all we need:
 - iterate through the whole file and print it's current char. simple as that.:
```c
int c; // our character will be granted as int, since fgetc() is able to return 257 values: -1 to 256 (EOF and 0 through all the ASCII TABLE)
while ((c = fgetc(FILE* file)) != EOF) // just as I told you, while it has not reached literally the End Of File (EOF), we print it
    printf("%c", c);
```

and that's it, really simple stuff, yet useful.
