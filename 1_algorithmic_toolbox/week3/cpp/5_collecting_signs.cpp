#include <algorithm>
#include <iostream>
#include <vector>

struct Period {
  int start, end;
};

std::vector<int> minimum_visits_fast(std::vector<Period> &periods)
{
  std::sort(periods.begin(), periods.end(),
    [](const Period &p1, const Period &p2) { return p1.end <= p2.end; });

  auto p_iter { periods.begin() };
  auto current_end { p_iter->end };
  std::vector<int> visits { current_end };
  while (++p_iter != periods.end())
    if (p_iter->start > current_end) {
      current_end = p_iter->end;
      visits.push_back(current_end);
    }

  return visits;
}

#ifndef STRESS_TEST

int main()
{
  int n {};
  std::cin >> n;
  std::vector<Period> periods;
  int start {}, end {};
  for (int i { 0 }; i < n; ++i) {
    std::cin >> start >> end;
    periods.push_back({ start, end });
  }

  const auto visits { minimum_visits_fast(periods) };
  std::cout << visits.size() << '\n';
  for (auto visit : visits) std::cout << visit << ' ';
  std::cout << '\n';
}

#else

 #include "stress_test.hpp"

std::ostream &operator<<(std::ostream &out, const Period &period)
{
  out << "Period(" << period.start << "," << period.end << ')';
  return out;
}

std::vector<int> minimum_visits_slow(std::vector<Period> periods)
/* Takes the argument vector by value (copy) instead of reference so that the original
 * vector is not used up by the slow function during stress testing
 * Will result in segmentation fault in the fast function otherwise
 */
{
  std::sort(periods.begin(), periods.end(),
    [](const Period &p1, const Period &p2) { return p1.end >= p2.end; });

  std::vector<int> visits;
  while (!periods.empty()) {
    const Period current { periods.back() };
    periods.pop_back();
    int overlap { current.end };

    auto periods_new_end { std::remove_if(
      periods.begin(), periods.end(), [&overlap, &current](const Period &period) {
        bool remove = current.start <= period.end && current.end >= period.start;
        if (remove) overlap = std::min(overlap, period.end);
        return remove;
      }) };
    periods.erase(periods_new_end, periods.end());

    visits.push_back(overlap);
  }

  std::sort(visits.begin(), visits.end());
  return visits;
}

int main()
{
  Rng<int> rng;
  rng.add_dist(1, 100);
  rng.add_dist(0, 1e9);
  sz n_tests { 0 };

  while (n_tests < 1e5) {
    sz n { static_cast<sz>(rng.from_dist(0)) };
    std::vector<Period> periods(n);
    for (auto &p : periods) {
      auto start { rng.from_dist(1) }, end { rng.from_dist(1) };
      if (start > end) std::swap(start, end);
      p = { start, end };
    }

    auto slow { minimum_visits_slow(periods) };
    auto fast { minimum_visits_fast(periods) };

    if (fast != slow) {
      std::cout << "\n\nTest failed!\nn = " << n << '\n';
      print_vec(periods);
      std::cout << "Result:\n (slow) ";
      print_vec(slow);
      std::cout << " (fast) ";
      print_vec(fast);
      break;
    }
    std::cout << "\rTests passed: " << ++n_tests;
  }
}

#endif
