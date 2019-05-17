all: program

program: a.rs
	rustc a.rs
	./a
