/// # C++ adapter example
///
/// C++ namespaces and classes map into the shared module and struct kinds.
namespace examples {

/// A small type with one documented method.
class Greeter {
public:
    /// Return a greeting for `name`.
    const char *greet(const char *name) const {
        return name;
    }
};

/// Add two integers.
int add_values(int left, int right);

} // namespace examples
