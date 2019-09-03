import os
import sys
import subprocess


class Color:
    CLEAR = "\033[0m"
    RED = "\033[1m\033[31m"
    GREEN = "\033[1m\033[32m"
    BLUE = "\033[1m\033[34m"


def make():
    test_files = sorted(os.listdir("./test"))
    f = open("test/expect.txt", "w")
    for idx, test_file in enumerate(test_files):
        if test_file == "expect.txt":
            continue
        f.write(f"{test_file} {expects[idx]}\n")
    f.close()


def test():
    f = open("test/expect.txt", "r")
    content = f.read()
    cases = {}
    cases = {
        line.split()[0]: int(line.split()[1])
        for line in content.split("\n")
        if len(line) > 0
    }
    for filename, expect in cases.items():
        p = subprocess.Popen(
            f"./target/debug/depth test/{filename} --intel -S > c.s; gcc c.s ; ./a.out",
            shell=True,
        )
        exit_status = p.wait()
        if exit_status != expect:
            print(
                f'[{filename}]{open(f"test/{filename}").read()} => {Color.RED}{expect} expected but got {exit_status}{Color.CLEAR}'
            )
            sys.exit(1)
        else:
            print(
                f'[{filename}]{open(f"test/{filename}").read()} => {Color.BLUE}{expect}{Color.CLEAR}'
            )
    print(f"{Color.GREEN}All Test Passed.{Color.CLEAR}")


if __name__ == "__main__":
    test()
    # make()
