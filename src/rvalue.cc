#include <iostream>
class Foo {
public:
    Foo();
};

Foo::Foo(){}

class Bar {
public:
    Bar(const Foo &foo);
    Bar(Foo &&foo);
};
Bar::Bar(const Foo &foo){
    std::cout << "bar copy called" << std::endl;
}
Bar::Bar(Foo &&foo) {
    std::cout << "bar move called" << std::endl;
};

class Derive: public Bar {
public:
    Derive(const Foo &&foo):Bar(foo){
        std::cout << "derive called" << std::endl;
    };
};

class Derive1: public Bar {
public:
    Derive1(const Foo &&foo): Bar(std::move(foo)) {
        std::cout << "derive1 called" << std::endl;
    }
};

class Derive2: public Bar{
public:
    Derive2(Foo &&foo): Bar(std::move(foo)) {
        std::cout << "derive2 called" << std::endl;
    }
};

int main() {
    Foo f;
    Derive d(std::move(f));

    // 仅仅一个const，d1 调用 copy constructor
    Foo f1;
    Derive1 d1(std::move(f));
    // d2 调用 move constructor
    Foo f2;
    Derive2 d2(std::move(f));
}
