#include <algorithm>
#include <iostream>
#include <vector>

long long maximum_ad_revenue(std::vector<int> &prices, std::vector<int> &clicks)
{
  std::sort(prices.begin(), prices.end());
  std::sort(clicks.begin(), clicks.end());
  long long total { 0 };
  for (int i { 0 }; i < prices.size(); ++i)
    total += static_cast<long long>(prices[i]) * clicks[i];

  return total;
}

int main()
{
  int n {};
  std::cin >> n;
  std::vector<int> prices(n, 0), clicks(n, 0);
  for (int i { 0 }; i < n; ++i) std::cin >> prices[i];
  for (int i { 0 }; i < n; ++i) std::cin >> clicks[i];

  std::cout << maximum_ad_revenue(prices, clicks) << '\n';
}
