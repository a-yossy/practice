#include <stdio.h>
typedef unsigned int uint;

void c_hello() {
  printf("Hello, world from C!\n");
}

uint c_fib(uint n) {
  if (n < 2) {
    return 1;
  } else {
    return c_fib(n - 1) + c_fib(n - 2);
  }
}
