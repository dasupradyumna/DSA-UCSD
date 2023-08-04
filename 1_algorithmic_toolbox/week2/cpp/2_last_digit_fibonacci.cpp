#include <iostream>

int fibonacci(int n)
{
  if (n <= 1) return n;

  int a { 0 }, b { 1 };
  while (--n) {
    b = a + b;
    a = b - a;
    b %= 10;
  }
  return b;
}

int main()
{
  int n {};
  std::cin >> n;
  std::cout << fibonacci(n) << '\n';
}
