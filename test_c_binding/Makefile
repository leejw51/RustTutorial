all:
	. ./build.sh
	gcc -g a.c  -o hi -lhello -L./hello/target/debug/
	LD_LIBRARY_PATH=./hello/target/debug ./hi

clean:
	rm -rf ./hello/target
