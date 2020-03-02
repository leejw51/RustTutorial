#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Fruit {
  int64_t price;
  int32_t (*call_back)(const uint8_t*);
} Fruit;

typedef int32_t (*MyCallback)(const uint8_t*);

void display(Fruit *f);

void set_callback(Fruit *f, MyCallback call_back);
