#include <iostream>
#include <vector>

using ll             = long long;
constexpr int MODULO = 10;

int last_digit_sum_fibonacci(const ll n)
{
  std::vector<int> cumulated_pisano;
  int a { 0 }, b { 1 }, sum { 0 };
  do {
    cumulated_pisano.push_back(sum);
    b = a + b;
    a = b - a;
    b %= MODULO;
    sum = (sum + a) % MODULO;
  } while (!(a == 0 && b == 1));

  ll p_sz { static_cast<ll>(cumulated_pisano.size()) };
  ll q { n / p_sz }, r { n % p_sz };
  return (q * cumulated_pisano[p_sz - 1] + cumulated_pisano[r]) % MODULO;
}

int main()
{
  ll n {};
  std::cin >> n;
  std::cout << last_digit_sum_fibonacci(n) << '\n';
}
