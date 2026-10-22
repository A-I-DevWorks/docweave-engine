/// C++ API documentation.
namespace api {
/// A generic-ish class with a nested declaration.
class Store {
public:
    /// Looks up a value using a complex parameter type.
    const std::vector<std::pair<int, std::string>> &find(
        const std::vector<int> &keys) const { return values_; }
private:
    std::vector<std::pair<int, std::string>> values_;
};
}

int undocumented_cpp(int value) { return value; }
