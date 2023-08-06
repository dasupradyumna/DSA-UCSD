#include <iostream>

long lcm(int a, int b)
{
  long lcm { static_cast<long>(a) * b };
  while (b) {
    auto t { a % b };
    a = b;
    b = t;
  }
  return lcm / a;
}

int main()
{
  int a {}, b {};
  std::cin >> a >> b;
  std::cout << lcm(a, b) << '\n';
}
