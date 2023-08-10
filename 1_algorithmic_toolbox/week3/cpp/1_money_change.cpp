#include <iostream>

int main()
{
  int n {};
  std::cin >> n;
  std::cout << (n / 10) + (n % 10 >= 5) + (n % 5) << '\n';
}
