package main
/*
#cgo CFLAGS: -I.. -g -Wall
#cgo LDFLAGS:  -lfruit  -L../target/debug/
#include <stdlib.h>
#include "fruit.h"
*/
import "C"
import "fmt"
import "unsafe"




func main() {
	s := C.add(200)
	fmt.Println(s)


	name := C.CString("Gopher")
	defer C.free(unsafe.Pointer(name))

	ptr := C.malloc(C.sizeof_char * 1024)
	defer C.free(unsafe.Pointer(ptr))

	size := C.add_text(name, (*C.uchar)(ptr), 1024)
	b := C.GoBytes(ptr, size)
	fmt.Println(string(b))
}
