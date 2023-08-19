#include <iostream>

int main()
{
  int n {}, a { 1 };
  std::cin >> n;
  while (n >= a) n -= a++;

  std::cout << --a << '\n';
  for (int i { 0 }; ++i < a;) std::cout << i << ' ';
  std::cout << a + n << '\n';
}
