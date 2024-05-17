#!/bin/bash

# Assemble the assembly file into an object file
as -o HelloWorld.o HelloWorld.s

# Link the object file into a final executable
ld -o HelloWorld -e _start HelloWorld.o -lSystem -syslibroot $(xcrun -sdk macosx --show-sdk-path) -arch arm64

# Run the executable
./HelloWorld
