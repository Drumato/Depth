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

try 9 "f main(){return 3+6}"
try 3 "f main(){return 6-3}"
try 2 "f main(){return 10-(3+5)}"
try 30 "f main(){return 3*10}"
try 7 "f main(){return 1+2*3}"
try 9 "f main(){return (1+2)*3}"
try 4 "f main(){return (-2)+6}"
try 2 "f main(){return 5%3}"
try 16 "f main(){return 1<<4}"
try 1 "f main(){return 16>>4}"
try 1 "f main(){return 1<2}"
try 0 "f main(){return 2<1}"
try 1 "f main(){return 6>4}"
try 0 "f main(){return 3>4}"
try 1 "f main(){return 6>=4}"
try 0 "f main(){return 2<=1}"
try 1 "f main(){return 1<=1}"
try 1 "f main(){return 1>=1}"
try 1 "f main(){return 1==1}"
try 0 "f main(){return 1==0}"
try 0 "f main(){return 1!=1}"
try 1 "f main(){return 1!=0}"
try 20 "f main(){if 1>0 return 20 return 30 }"
try 30 "f main(){if 0>1 return 20 return 30 }"
try 20 "f main(){if 1 return 20 return 30 }"
try 30 "f main(){if 0 return 20 return 30 }"
echo -e "\e[33mAll Test Passed.\e[0m"

make clean
