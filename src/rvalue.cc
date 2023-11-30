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

int main() {
    Foo f;
    Derive d(std::move(f));

    Foo f1;
    Derive1 d1(std::move(f));
}