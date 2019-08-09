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

try 41 'f main(){return 12 + 34 - 5}'
try 47 "f main(){return 5+6*7}"
try 6 'f main(){let x : i8 = 3 return x+3}'
try 6 'f main(){let foo : i8 = 3 let bar : i8 = 3 return foo+bar}'
try 30 'f main(){let a : i8 = 1 if a > 0{ return 30 }}'
try 30 'f main(){let a : i8 = 1 if a >= 1 { return 30 } }'
try 30 'f main(){let a : i8 = 1 if a <= 1 { return 30 } }'
try 30 'f main(){let a : i8 = 1 if a == 1 { return 30 } }'
try 30 'f main(){let a : i8 = 1 if a != 0 { return 30 } }'
try 50 'f main(){let a : i8 = 0 if a > 0{ return 30 } else { return 50 } }'
try 30 'f main(){let a : i8 = 1 if a > 0{ return 30 } else { return 50 } }'
try 30 'f main(){let a : i8 = 1 if a >= 1 { return 30 } else { return 50 } }'
try 30 'f main(){let a : i8 = 1 if a <= 1 { return 30 } else { return 50 } }'
try 30 'f main(){let a : i8 = 1 if a == 1 { return 30 } else { return 50 } }'
try 30 'f main(){let a : i8 = 1 if a != 2 { return 30 } else { return 50 } }'
try 30 'f main(){ return add()} f add() -> i8 { return 30}'
echo -e "\e[33mAll Test Passed.\e[0m"

make clean
