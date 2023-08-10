#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

using item = std::pair<double, int>;

double maximum_cost_capacity(int capacity, std::vector<item> &rates_weights)
{
  std::sort(rates_weights.begin(), rates_weights.end(),
    [](const item &left, const item &right) { return left.first > right.first; });

  double total { 0 };
  for (auto rate_weight : rates_weights) {
    if (capacity <= 0) break;
    total += rate_weight.first * std::min(rate_weight.second, capacity);
    capacity -= rate_weight.second;
  }
  return total;
}

int main()
{
  int n {}, capacity {};
  std::cin >> n >> capacity;
  std::vector<item> rates_weights(n);
  double rate {};
  int weight {};
  for (int i { 0 }; i < n; ++i) {
    std::cin >> rate >> weight;
    if (weight > 0) rate /= weight;
    rates_weights.push_back({ rate, weight });
  }

  std::cout << std::fixed << std::setprecision(4);
  std::cout << maximum_cost_capacity(capacity, rates_weights) << '\n';
}
