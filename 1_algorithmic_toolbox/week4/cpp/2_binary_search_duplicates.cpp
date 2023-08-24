#include <iostream>
#include <vector>

int binary_search_duplicates(const std::vector<int> &sequence, const int query)
{
  int left { 0 }, right { static_cast<int >(sequence.size()) }, mid {};
  while (mid = left + (right - left) / 2, left < right)
    if (sequence[mid] == query && (mid == 0 || sequence[mid - 1] != query))
      return mid;
    else if (sequence[mid] >= query)
      right = mid;
    else
      left = mid + 1;

  return -1;
}

int main()
{
  int n {}, m {}, query {};
  std::cin >> n;
  std::vector<int> sequence(n);
  for (auto &e : sequence) std::cin >> e;
  std::cin >> m;
  for (int i { 0 }; i < m; ++i) {
    std::cin >> query;
    std::cout << binary_search_duplicates(sequence, query) << ' ';
  }
  std::cout << std::endl;
}
