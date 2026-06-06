#!/bin/bash

SRC=main.c

if [ -f "$SRC" ]; then
	gcc "$SRC" -o tmp.o
	./tmp.o "$1"
	rm ./tmp.o
fi
