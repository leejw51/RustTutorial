all: go_language 


c_language:
	cargo build
	gcc -g test.c -o test -lfruit  -L../target/debug/
	LD_LIBRARY_PATH=../target/debug ./test


go_language:
	go build test_go.go
	LD_LIBRARY_PATH=../target/debug ${PWD}/test_go

