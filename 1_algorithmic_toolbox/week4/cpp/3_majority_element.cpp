#include <algorithm>
#include <iostream>
#include <vector>

bool majority_element(std::vector<int> &sequence)
{
  int n_maj { -1 }, curr { -1 }, n_curr { -1 };

  std::sort(sequence.begin(), sequence.end());
  for (const auto &e : sequence)
    if (e == curr)
      ++n_curr;
    else {
      n_maj  = std::max(n_maj, n_curr);
      curr   = e;
      n_curr = 1;
    }

  return std::max(n_maj, n_curr) > sequence.size() / 2;
}

int main()
{
  int n {};
  std::cin >> n;
  std::vector<int> sequence(n);
  for (auto &e : sequence) std::cin >> e;

  std::cout << majority_element(sequence) << '\n';
}
