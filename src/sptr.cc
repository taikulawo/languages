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

class Bar1 {
  public:
    Bar1(Foo &foo) {}
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
    Foo foo1 = Foo();
    Foo const & foo2 = Foo();
    // https://stackoverflow.com/questions/73029858/c-does-passing-by-reference-utilize-implicit-conversion
    // C++ reference 只能在初始化时绑定
    // Bar1 接受 &t，编译器会隐式将foo1转换为 reference 再传递
    Bar1 bar2 = Bar1(foo1);
    // won't compile
    // Bar1 bar3 = Bar1(foo2);
    COUT("endof scope");
    return 0;
} 