#include <iostream>
#include <vector>

using ll             = long long;
constexpr int MODULO = 10;

int last_digit_sum(const ll n, const std::vector<int> &cp)
{
  ll p_sz { static_cast<ll>(cp.size()) };
  ll q { n / p_sz }, r { n % p_sz };
  return (q * cp[p_sz - 1] + cp[r]) % MODULO;
}

int last_digit_partial_sum_fibonacci(const ll m, const ll n)
{
  std::vector<int> cp;  // cumulated pisano modulos
  int a { 0 }, b { 1 }, sum { 0 };
  do {
    cp.push_back(sum);
    b = a + b;
    a = b - a;
    b %= MODULO;
    sum = (sum + a) % MODULO;
  } while (!(a == 0 && b == 1));

  return m == 0 ? last_digit_sum(n, cp)
                : (last_digit_sum(n, cp) - last_digit_sum(m - 1, cp) + MODULO) % MODULO;
}

int main()
{
  ll m {}, n {};
  std::cin >> m >> n;
  std::cout << last_digit_partial_sum_fibonacci(m, n) << '\n';
}
