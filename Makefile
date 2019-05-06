depth:
	make clean
	go build depth.go && ./test.sh

clean:
	rm -f depth *.o tmp* *.out *.txt *.s

doc:
	golint -set_exit_status $$(go list ./...)
	go vet ./...
