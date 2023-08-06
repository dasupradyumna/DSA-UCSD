#include <iostream>
#include <vector>

using ll             = long long;
constexpr int MODULO = 10;

int last_digit_sum_squares_fibonacci(const ll n)
{
  std::vector<int> pisano10;
  int a { 0 }, b { 1 };
  do {
    pisano10.push_back(a);
    b = a + b;
    a = b - a;
    b %= MODULO;
  } while (!(a == 0 && b == 1));

  auto p_sz { pisano10.size() };
  return (pisano10[n % p_sz] * pisano10[(n + 1) % p_sz]) % MODULO;
}

int main()
{
  ll n {};
  std::cin >> n;
  std::cout << last_digit_sum_squares_fibonacci(n) << '\n';
}
