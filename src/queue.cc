#include <cassert>
#include <iostream>
#include <queue>

int main() {
    std::queue<int> q;
    q.push(0);
    q.push(1);
    q.push(2);
    assert(q.front() == 0);
    assert(q.back() == 2);
    assert(q.size() == 3);

    q.pop();
    assert(q.size() == 2);

    std::cout << "q: ";
    for (; !q.empty(); q.pop())
        std::cout << q.front() << ' ';
    std::cout << '\n';
    assert(q.size() == 0);
}