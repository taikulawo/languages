#include <stdio.h>
#include <string.h>
#include <vector>
#include <iostream>
#include <memory>
void test_const_reference_lvalue(const std::string &name) {}
void test_rvalue(std::string &&name) {}
using StringPtr = std::unique_ptr<std::string>;
namespace Listener {

std::string version;
class Manager {
  public:
    std::string name;
    Manager(std::string &&name) {
        std::cout << "rvalue call" << std::endl;
        this->name = std::move(name);
    }
    Manager(StringPtr &&name) {
        std::cout << "std::unique_ptr<str::string>&& name called" << std::endl;
    }
    Manager(const std::string &name) {
        std::cout << "const reference call" << std::endl;
        this->name = name;
    }
    Manager(std::string &name) {}
};
class Manager2 {
  public:
    std::string name;
    Manager2(const std::string &name) {
        std::cout << "const reference call" << std::endl;
        this->name = name;
    }
};

class Listener {
  public:
    using ManagerPtr = std::unique_ptr<Manager>;
    ManagerPtr manager;
    Manager manager1;
    Listener() : manager1(std::move(std::string("hello"))) {
        this->manager =
            std::make_unique<Manager>(std::move(std::string("hello")));
    }
    void show() { std::cout << this->manager->name << std::endl; }
};

std::vector<Manager> managers = {{std::string("123")},
                                 {
                                     std::string("345"),
                                 }};

std::unique_ptr<Manager> manager = std::make_unique<Manager>("");
}; // namespace Listener

int main(int argc, const char **argv) {
    std::string url = "127.0.0.1:443/v1/lookup/name?name=a.b.c";
    std::string s = url.substr(url.find(':'));
    int port;
    sscanf(s.c_str() + 1, "%d", &port);
    printf("%d\n", port);

    // vector for auto loop
    std::vector<int> child(1);
    for (auto &c : child) {
        c = 6;
    }
    std::vector<int>::iterator it;
    for (it = child.begin(); it != child.end(); it++) {
        std::cout << *it << std::endl;
    }
    Listener::version = argv[0];
    std::cout << Listener::version << std::endl;

    for (auto it = Listener::managers.begin(); it != Listener::managers.end();
         it++) {
        std::cout << it->name << std::endl;
    }
    // Manager2 没 string &&name构造函数，会调用到 const std::string& name
    // 说明 rvalue可以bind到 const reference parameter
    Listener::Manager2 m = Listener::Manager2("manager 2");
    std::vector<std::vector<int>> n = {{1, 2, 3}, {4, 5, 6}};
    for (auto &v : n) {
        for (auto it = v.begin(); it != v.end(); it++) {
            std::cout << *it << std::ends;
        }
        std::cout << std::endl;
    }
    // rvalue向普通引用（lvalue）转换
    test_const_reference_lvalue(std::move("hello"));
    // ""是临时变量，ELF只读区，右值
    test_const_reference_lvalue("");
    // rvalue
    test_rvalue(std::move("hello"));
    // ""是rvalue，意指等号右侧的值，大多数场景只能读取，无法写入，比如函数调用，常量
    test_rvalue("");
    Listener::Manager m1 =
        Listener::Manager(std::make_unique<std::string>(""));
    Listener::Manager m2 =
        Listener::Manager(std::move(std::make_unique<std::string>("")));
    return 0;
}