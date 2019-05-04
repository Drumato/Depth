depth:
	go build depth.go 

clean:
	rm -f depth *.o tmp* *.out *.txt

doc:
	golint -set_exit_status $$(go list ./...)
	go vet ./...
