#include <algorithm>
#include <iostream>
#include <numeric>
#include <vector>

int main()
{
  int n {};
  std::cin >> n;
  std::vector<std::string> sequence(n);
  for (auto &str : sequence) std::cin >> str;

  std::sort(sequence.begin(), sequence.end(),
    [](const std::string &s1, const std::string &s2) { return s1 + s2 >= s2 + s1; });

  std::string concat {};
  for (const auto &str : sequence) concat += str;
  std::cout << concat << '\n';
}
