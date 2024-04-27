#include <stdio.h>
#include <cstdint>
#include <variant>

class Foo {
  public:
    int32_t a;
    int32_t b;
    int32_t c;
    int32_t d;
};

struct Bar {
    int32_t a;
    int32_t b;
    int32_t c;
    int32_t d;
};

struct Foo0 {
    char a;
    uint64_t b;
};

int main() {
    std::variant<int, Foo> f;
    std::variant<int, Bar> f1;
    // Foo 16字节，variant需要额外空间存储active type index
    // 具体多少取决实现
    // https://stackoverflow.com/a/53686803/7529562
    printf("sizeof Foo %lu, sizeof f is %lu, sizeof int %lu\n", sizeof(Foo),
           sizeof(f), sizeof(int));
    printf("sizeof f1 is %lu\n", sizeof(f1));
    printf("sizeof Foo0 is %lu\n", sizeof(Foo0));
    return 0;
}