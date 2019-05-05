#!/bin/bash
try() {
  expected="$1"
  input="$2"

  ./depth --until-compile "$input" > tmp.s
  gcc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" == "$expected" ]; then
    echo -e "$input \e[32m=> $actual\e[0m"
  else
    echo "$input: $expected expected, but got $actual"
    make clean
    exit 1
  fi
}

try 100 'f main(){30-20+40+50}'

echo -e "\e[32mOK\e[0m"
make clean
