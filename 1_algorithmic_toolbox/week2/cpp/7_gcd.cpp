#include <iostream>

int gcd(int a, int b)
{
  while (b) {
    auto t { a % b };
    a = b;
    b = t;
  }
  return a;
}

int main()
{
  int a {}, b {};
  std::cin >> a >> b;
  std::cout << gcd(a, b) << '\n';
}
