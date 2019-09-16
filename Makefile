all:
	cargo test
	cargo build && ./test.sh
	make clean
gcc:
	cargo build
	dr --intel sample.dep
fclean:
	cargo clean
	rm -f *.o tmp* *.out *.txt *.s
clean:
	rm -f ./target/debug/depth *.o tmp* *.out *.txt *.s test/testc/*.s 
