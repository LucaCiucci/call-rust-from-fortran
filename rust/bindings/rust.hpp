#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class SomeEnum {
  SomeEnum_A,
  SomeEnum_B,
};

/// Some Rustacean struct
struct rustacean_struct {
  int number;
  float pi;
  SomeEnum e;
};

extern "C" {

/// Greet from Rust
void hello_from_rust();

/// Sum two numbers
int rustacean_sum(int a, int b);

/// Same as [rustacean_sum] but with pointers
void display_rustacean_struct(const rustacean_struct *s);

} // extern "C"
