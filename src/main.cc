#include <stdio.h>
#include <string.h>
#include <vector>
#include <iostream>
std::string version;
int main(int argc, const char **argv) {
    std::string url = "127.0.0.1:443/v1/lookup/name?name=a.b.c";
    std::string s = url.substr(url.find(':'));
    int port;
    sscanf(s.c_str() + 1, "%d", &port);
    printf("%d\n", port);

    // vector for auto loop
    std::vector<int> child(20);
    for (auto & c : child) {
        c = 1;
    }
    std::vector<int>::iterator it;
    for (it = child.begin(); it != child.end(); it ++) {
        std::cout << *it << std::endl;
    }
    version = argv[0];
    std::cout << version << std::endl;
    return 0;
}