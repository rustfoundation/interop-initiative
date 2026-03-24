#include <cstdlib>
#include <iostream>

extern "C" void free_in_cpp(uint32_t* ptr ,size_t len) {
  std:: cout << "C++ received pointer and attempting to free..." << std::endl;

  // Wrong bcz memory is allocated in Rust not C++
  free(ptr);
  std::cout << "Freed in C++ (undefined behaviour!)" << std::endl;
}