#!/bin/bash
# Use this to compile the test patch dll for the game

gcc -o ../patch.dll -s -shared dll_testing.c -ldl
