#!/bin/bash
cargo build
try() {
  expected="$1"
  input="$2"

  ./target/debug/depth --intel "$input" > tmp.s
  gcc -static -o tmp tmp.s
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

try 9 "return 3+6"
try 3 "return 6-3"
try 2 "return 10-(3+5)"
try 30 "return 3*10"
try 7 "return 1+2*3"
try 9 "return (1+2)*3"
try 4 "return (-2)+6"
try 2 "return 5%3"
try 16 "return 1<<4"
try 1 "return 16>>4"
try 1 "return 1<2"
try 0 "return 2<1"
try 1 "return 6>4"
try 0 "return 3>4"
try 1 "return 6>=4"
try 0 "return 2<=1"
try 1 "return 1<=1"
try 1 "return 1>=1"
try 1 "return 1==1"
try 0 "return 1==0"
try 0 "return 1!=1"
try 1 "return 1!=0"
echo -e "\e[33mAll Test Passed.\e[0m"

make clean
