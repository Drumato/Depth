all:
	cargo test
	cargo build && ./test.sh
	make clean
gcc:
	cargo build
	dr --intel sample.dep
fclean:
	cargo clean
	rm -f depth *.o tmp* *.out *.txt *.s
clean:
	rm -f depth *.o tmp* *.out *.txt *.s
