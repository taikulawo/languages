#include <vector>
#include <memory>
#include "filter/filter.h"

int main() {
    std::vector<int> v{1, 2, 3, 4, 5};
    std::vector<int> w(std::move(v));
    v.push_back(6);
    w.push_back(7);
    return 0;
}

class Filter {
  private:
    CodecFactoryPtr codec_factory;
};

