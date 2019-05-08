#!/bin/bash
try() {
  expected="$1"
  input="$2"

  ./depth --until-compile "$input"
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

try 100 'f main(){ return 30-20+40+50}'
try 25 'f main(){ return 6*2+30/2-2}'
try 10 'f main(){ let x:i8 = 10 return x}'
try 65 "f main(){ let c:ch = 'A' return c}"

echo -e "\e[32mOK\e[0m"
make clean
