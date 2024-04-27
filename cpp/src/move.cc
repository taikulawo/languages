#include <vector>
#include <memory>
#include "filter/filter.h"

int main() {
    std::vector<int> v{1, 2, 3, 4, 5};
    std::vector<int> w(std::move(v));
    // UB after move
    v.push_back(6);
    w.push_back(7);

    std::unique_ptr<NetworkFilters::CodecFactory> codec =
        std::make_unique<NetworkFilters::CodecFactoryImpl>();
    std::unique_ptr<NetworkFilters::Filter> p =
        std::make_unique<NetworkFilters::Filter>(std::move(codec));
    p->request();
    return 0;
}
