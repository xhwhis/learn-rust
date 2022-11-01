#include <stdio.h>

extern int add(int a, int b) { return a + b; }
extern int sub(int a, int b) { return a - b; }

extern void register_callback(int(int, int));
extern int trigger_callback();

int main() {
  register_callback(add);
  printf("add(1, 2) = %d\n", trigger_callback());
  register_callback(sub);
  printf("sub(1, 2) = %d\n", trigger_callback());
  return 0;
}
