#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>
#include <assert.h>
#include "../fruit.h"

int test_callback(char* msg)  {
    printf("*********************callback %s\n",msg);
    printf("#########################\n");
    return 0;
}

int main()
{
    Fruit* f= (Fruit*) malloc(sizeof(Fruit));
    f->price=200;
    f->call_back = &test_callback;
    while(true) {
        display(f);
        sleep(1);
        f->price++;
    }
    
    free(f);
    return 0;
}
