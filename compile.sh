#!/usr/bin/bash
set -e

mkdir -p output

echo "Compiling $1.S -> $1.o"
as "output/$1.S" -o "output/$1.o"

echo "Compiling $1.o -> $1"
gcc "output/$1.o" -o "output/$1"
