#!/bin/bash

SRC=main.c
OBJ=grep
# provide ./run.sh with two arguments:
# $1 => file
# $2 => input

if [ ! -f "$SRC" ]; then
	echo -e "Correct usage: ./run.sh \033[3m{file} {input}\033[0m"
	exit 1
fi

if [ ! -f "$1" ]; then
	echo -e "Couldn't find \033[3m./"$1"\033[0m"
	exit 1
fi

if [ -z "$2" ]; then
	echo -e "You're missing the input: ./run.sh \033[3m{file} {input}\033[0m"
fi

gcc "$SRC" -o "$OBJ" && ./"$OBJ" "$1" "$2"
rm "$OBJ"
