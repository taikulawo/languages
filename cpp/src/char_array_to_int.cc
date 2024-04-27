#include <string>
#include <cstdint>
#include <iostream>
#include <cmath>
int main(int argc, char *argv[]) {
    std::string s = "123";
    uint64_t value = 0;
    for (int i = 0; i < s.size(); i++) {
        value += (s[i] - '0') * std::pow(10, s.size() - 1 - i);
    }
    std::cout << value << std::endl;
}