#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Fruit {
  int64_t price;
  int32_t (*call_back)(const uint8_t*);
} Fruit;

void display(Fruit *f);
