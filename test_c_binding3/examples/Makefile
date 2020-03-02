all:
	cargo build
	gcc -g test.c -o test -lfruit  -L../target/debug/
	LD_LIBRARY_PATH=../target/debug ./test
