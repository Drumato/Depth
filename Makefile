all:
	cargo test
	cargo build && ./test.sh
	make clean
fclean:
	cargo clean
	rm -f depth *.o tmp* *.out *.txt *.s
clean:
	rm -f depth *.o tmp* *.out *.txt *.s
