#include <iostream>
#include <vector>

int fibonacci_mod_m(const long long n, const int m)
{
  std::vector<int> pisano_period;
  int a { 0 }, b { 1 };
  do {
    pisano_period.push_back(a);
    b = a + b;
    a = b - a;
    b %= m;
  } while (!(a == 0 && b == 1));

  return pisano_period[n % pisano_period.size()];
}

int main()
{
  long long n {};
  int m {};
  std::cin >> n >> m;
  std::cout << fibonacci_mod_m(n, m) << '\n';
}
