#include <iostream>
#include <memory>

class Foo {
  public:
    ~Foo() { std::cout << "Foo destructor func called" << std::endl; }
};

int main() {
    { auto foo = std::make_unique<Foo>(); }
    std::cout << "endof scope" << std::endl;
    return 0;
}