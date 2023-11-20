#include <memory>

class CodecFactory {
    virtual void requestDecoder() = 0;
};

using CodecFactoryPtr = std::unique_ptr<CodecFactory>;
