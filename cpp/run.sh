#!/bin/bash
clang++ main.cpp -Wall -o main.exe -std=c++17 -pedantic-errors
exitcode=$?
RED='\033[0;31m'
NC='\033[0m'
if [ $exitcode -ne 0 ]; then
	echo -e "${RED}compilation failed${NC}"
	exit 1
fi
echo compiled
./main.exe < input.txt
