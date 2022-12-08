#include <stdio.h>

int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }

extern void register_callback(int(int, int));
extern int trigger_callback(char *);

int main() {
  register_callback(add);
  printf("add(1, 2) = %d\n", trigger_callback("func(1, 2) - 3"));
  register_callback(sub);
  printf("sub(1, 2) = %d\n", trigger_callback("func(1, 2) + 3"));
  return 0;
}
