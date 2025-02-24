#include <stdio.h>

typedef unsigned int unit;

unit c_fib(unit n) {
  if (n < 2) {
    return 1;
  } else {
    return c_fib(n - 1) + c_fib(n - 2);
  }
}
