import os
import sys
import subprocess
import time


class Color:
    CLEAR = "\033[0m"
    RED = "\033[1m\033[31m"
    GREEN = "\033[1m\033[32m"
    BLUE = "\033[1m\033[34m"


def make():
    test_files = sorted(os.listdir("./test/"))
    f = open("test/expect.txt", "w")
    for idx, test_file in enumerate(test_files):
        if test_file == "expect.txt":
            continue
        f.write(f"{test_file} {expects[idx]}\n")
    f.close()


def test_compile():
    print(f"{Color.GREEN}++++++++++++++++test-votalile++++++++++++++++{Color.CLEAR}")
    f = open("test/expect.txt", "r")
    content = f.read()
    cases = {}
    cases = {
        line.split()[0]: int(line.split()[1])
        for line in content.split("\n")
        if len(line) > 0
    }
    for filename, expect in cases.items():
        fn = f"test/{filename}"
        f = open(fn)
        p = subprocess.Popen(f"./target/debug/depth {fn} --run", shell=True)
        exit_status = p.wait()
        if exit_status != expect:
            print(
                f"[{filename}]{f.read()} => {Color.RED}{expect} expected but got {exit_status}{Color.CLEAR}"
            )
            sys.exit(1)
        else:
            print(f"[{filename}] => {Color.BLUE}{expect}{Color.CLEAR}")
    print(f"{Color.GREEN}All Test Passed.{Color.CLEAR}")


def test_optimize1():
    print(f"{Color.GREEN}++++++++++++++++test-optimize1++++++++++++++++{Color.CLEAR}")
    f = open("test/expect.txt", "r")
    content = f.read()
    cases = {}
    cases = {
        line.split()[0]: int(line.split()[1])
        for line in content.split("\n")
        if len(line) > 0
    }
    for filename, expect in cases.items():
        fn = f"test/{filename}"
        f = open(fn)
        p = subprocess.Popen(f"./target/debug/depth {fn} --Opt1 ; ./a.out", shell=True)
        exit_status = p.wait()
        if exit_status != expect:
            print(
                f"[{filename}]{f.read()} => {Color.RED}{expect} expected but got {exit_status}{Color.CLEAR}"
            )
            sys.exit(1)
        else:
            print(f"[{filename}] => {Color.BLUE}{expect}{Color.CLEAR}")
    print(f"{Color.GREEN}All Test Passed.{Color.CLEAR}")


def test_llvm():
    print(f"{Color.GREEN}++++++++++++++++test-llvm++++++++++++++++{Color.CLEAR}")
    f = open("test/expect.txt", "r")
    content = f.read()
    cases = {}
    cases = {
        line.split()[0]: int(line.split()[1])
        for line in content.split("\n")
        if len(line) > 0
    }
    for filename, expect in cases.items():
        fn = f"test/{filename}"
        f = open(fn)
        p = subprocess.Popen(
            f"./target/debug/depth {fn} --emit-llvm  > d.ll ; clang d.ll; ./a.out",
            shell=True,
        )
        exit_status = p.wait()
        if exit_status != expect:
            print(
                f"[{filename}]{f.read()} => {Color.RED}{expect} expected but got {exit_status}{Color.CLEAR}"
            )
            sys.exit(1)
        else:
            print(f"[{filename}] => {Color.BLUE}{expect}{Color.CLEAR}")
    print(f"{Color.GREEN}All Test Passed.{Color.CLEAR}")


if __name__ == "__main__":
    start = time.time()
    test_compile()
    compile_time = time.time() - start
    print(f"test-volatile time -> {Color.BLUE}{round(compile_time,2)}{Color.CLEAR}s")
    # start = time.time()
    # test_optimize1()
    # compile_time = time.time() - start
    # print(f"test-optimize1 time -> {Color.BLUE}{round(compile_time,2)}{Color.CLEAR}s")
    start = time.time()
    test_llvm()
    compile_time = time.time() - start
    print(f"test-llvm time -> {Color.BLUE}{round(compile_time,2)}{Color.CLEAR}s")
    # make()
