#include <iostream>

int fibonacci(int n)
{
  if (n <= 1) return n;

  int a { 1 }, b { 0 };
  while (--n) {
    a = a + b;
    b = a - b;
    a %= 10;
  }
  return a;
}

int main()
{
  int n {};
  std::cin >> n;
  std::cout << fibonacci(n) << '\n';
}
