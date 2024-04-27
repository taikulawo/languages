#include <memory>
#include <iostream>

namespace NetworkFilters {
class CodecFactory {
  public:
    virtual void requestDecoder() = 0;
};

class CodecFactoryImpl : public CodecFactory {
  public:
    CodecFactoryImpl(){};

  private:
    void requestDecoder() override {
        std::cout << "request decoder" << std::endl;
    }
};

using CodecFactoryPtr = std::unique_ptr<CodecFactory>;
class Filter {
  public:
    Filter(CodecFactoryPtr codec) : codec_factory(std::move(codec)){};
    void request() { this->codec_factory->requestDecoder(); }

  private:
    CodecFactoryPtr codec_factory;
};

} // namespace NetworkFilters
