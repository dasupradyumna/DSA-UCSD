#include <iostream>
#include <tuple>
#include <vector>

using sz = std::size_t;

sz pivot(const std::vector<int> &seq, sz lo, sz hi)
{
  auto median = [&seq](sz a, sz b, sz c) {
    return (seq[b] <= seq[a] && seq[a] <= seq[c]) ? a
         : (seq[a] <= seq[b] && seq[b] <= seq[c]) ? b
                                                  : c;
  };

  const sz d = hi - lo;
  sz pivot   = lo + d / 2;
  if (d >= 50) {
    if (d >= 100) {
      lo    = median(lo, lo + d / 8, lo + d / 4);
      pivot = median(pivot - d / 8, pivot, pivot + d / 8);
      hi    = median(hi - d / 4, hi - d / 8, hi);
    }
    pivot = median(lo, pivot, hi);
  }

  return pivot;
}

std::pair<sz, sz> part3way(std::vector<int> &seq, const sz lo, const sz hi)
{
  std::swap(seq[lo], seq[pivot(seq, lo, hi - 1)]);
  const auto pivot = seq[lo];

  sz iter { lo + 1 }, lesser { iter }, greater { hi - 1 };
  while (iter <= greater)
    if (seq[iter] > pivot)
      std::swap(seq[iter], seq[greater--]);
    else if (seq[iter] == pivot)
      ++iter;
    else
      std::swap(seq[iter++], seq[lesser++]);

  std::swap(seq[lo], seq[--lesser]);
  return { lesser, ++greater };
}

void quick_sort(std::vector<int> &seq)
{
  std::vector<std::pair<sz, sz>> stack { { 0, seq.size() } };

  while (!stack.empty()) {
    long lo, hi;
    std::tie(lo, hi) = stack.back();
    stack.pop_back();
    if (hi - lo <= 1) continue;

    long p1, p2;
    std::tie(p1, p2) = part3way(seq, lo, hi);
    stack.emplace_back(lo, p1);
    stack.emplace_back(p2, hi);
  }
}

int main()
{
  int n {};
  std::cin >> n;
  std::vector<int> seq(n);
  for (auto &e : seq) std::cin >> e;
  quick_sort(seq);

  for (const auto &e : seq) std::cout << e << ' ';
  std::cout << std::endl;
}
