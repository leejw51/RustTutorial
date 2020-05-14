package main
/*
#cgo CFLAGS: -I.. -g -Wall
#cgo LDFLAGS:  -lfruit  -L../target/debug/
#include <stdlib.h>
#include "fruit.h"
*/
import "C"
import "fmt"




func main() {
	s := C.add(200)
	fmt.Println(s)
}
