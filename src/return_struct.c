#include <stdlib.h>
#include <stdio.h>

typedef struct {
    void *p;
    int len;
} req;

req foo() {
    req x;
    x.p = malloc(10);
    x.len = 1;
    return x;
}

int main() {
    req r = foo();
    printf("%p", r.p);
    return 0;
}