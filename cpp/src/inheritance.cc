
#include <stdio.h>
class Base {
  public:
    void f() {
        void (Base::*p)(void) = &Base::do_f;
        printf("base do_f address %p", p);
        this->do_f();
    };

  private:
    virtual void do_f() { printf("base do_f"); }
};
class Derive : public Base {
    void do_f() override { printf("derive do_f"); }
};

int main() {
    Derive d;
    Base *p = &d;
    // 输出 derive do_f
    p->f();

    return 0;
}