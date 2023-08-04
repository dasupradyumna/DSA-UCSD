#include <functional>
#include <iostream>
#include <numeric>

using ll = long long;
using sz = unsigned long;

ll maximum_pairwise_product_slow(const std::vector<int> &sequence)
{
  ll result { 0 };
  sz n { sequence.size() };

  for (sz first { 0 }; first < n; ++first)
    for (sz second { first + 1 }; second < n; ++second)
      result = std::max(result, static_cast<ll>(sequence[first]) * sequence[second]);

  return result;
}

ll maximum_pairwise_product_fast(const std::vector<int> &sequence)
{
  sz max1 { 0 }, max2 { 1 }, n { sequence.size() };
  if (sequence[max1] < sequence[max2]) std::swap(max1, max2);

  for (sz idx { 2 }; idx < n; ++idx)
    if (sequence[idx] > sequence[max1]) {
      max2 = max1;
      max1 = idx;
    } else if (sequence[idx] > sequence[max2])
      max2 = idx;

  return static_cast<ll>(sequence[max1]) * sequence[max2];
}

ll maximum_pairwise_product_fastest(const std::vector<int> &sequence)
{
  std::function<std::vector<int>(sz, sz)> find_largest;
  find_largest = [&sequence, &find_largest](sz begin, sz end) {
    if (begin == end) return std::vector<int> { sequence[begin] };

    auto left { find_largest(begin, (begin + end) / 2) };
    auto right { find_largest((begin + end) / 2 + 1, end) };

    if (left[0] >= right[0]) {
      left.push_back(right[0]);
      return left;
    } else {
      right.push_back(left[0]);
      return right;
    }
  };

  auto largest_path { find_largest(0, sequence.size() - 1) };
  int second_largest { std::accumulate(largest_path.begin() + 1, largest_path.end(), -1,
    [](int res, int val) { return std::max(res, val); }) };

  return static_cast<ll>(largest_path[0]) * second_largest;
}

#ifdef STRESS_TEST

 #include "stress_test.hpp"

int main()
{
  RNG<int> rng;
  rng.add_dist(2, 1e3);
  rng.add_dist(0, 2e5);
  ll n_tests { 0 };

  while (n_tests < 1e5) {
    sz n { static_cast<sz>(rng.from_dist(0)) };
    auto sequence { rng.create_vec(n, 1) };

    ll slow { maximum_pairwise_product_slow(sequence) };
    ll fast { maximum_pairwise_product_fastest(sequence) };

    if (fast != slow) {
      std::cout << "\n\nTest failed!\nn = " << n << "\n[ ";
      print_vec(sequence);
      std::cout << "]\nResult: (slow) " << slow << ", (fast) " << fast << '\n';
      break;
    }
    std::cout << "\rTests passed: " << ++n_tests;
  }
}

#else

int main()
{
  int n {};
  std::cin >> n;
  std::vector<int> sequence(n);
  for (int i { 0 }; i < n; ++i) std::cin >> sequence[i];

  std::cout << maximum_pairwise_product_fastest(sequence) << '\n';
}

#endif
