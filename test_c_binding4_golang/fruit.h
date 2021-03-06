#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Fruit {
  int64_t price;
  int32_t (*call_back)(const uint8_t*);
} Fruit;

typedef int32_t (*MyCallback)(const uint8_t*);

int32_t add(int32_t f);

int32_t add_text(const char *src, uint8_t *output, uint32_t output_length);

void display(Fruit *f);

void set_callback(Fruit *f, MyCallback call_back);
