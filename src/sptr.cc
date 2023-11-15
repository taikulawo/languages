#include <iostream>
#include <memory>
#define COUT(str) std::cout << '\n' << str << std::endl
#define PRINT(...) std::cout << #__VA_ARGS__ << " = " << __VA_ARGS__ << '\n'

class Foo {
  public:
    ~Foo() { std::cout << "Foo destructor func called" << std::endl; }
};

class Bar : public Foo {
  public:
    ~Bar() { COUT("Bar destructor func called"); }
};

int main() {
    auto foo = std::make_unique<Foo>();
    auto bar = std::make_shared<Bar>();
    // cpp = 是copy
    // rust除非实现Copy trait，否则是move
    std::shared_ptr<Bar> bar1 = bar;
    PRINT(bar.use_count());
    {
      std::shared_ptr<Bar> bar2 = bar1;
      PRINT(bar.use_count());
    }
    PRINT(bar.use_count());
    COUT("endof scope");
    return 0;
}