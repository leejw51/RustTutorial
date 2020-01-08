#!/bin/bash
nm --defined-only  -D ./hello/target/debug/libhello.so
ldd ./hello/target/debug/libhello.so
