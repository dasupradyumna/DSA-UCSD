#include <iostream>
#include <vector>

int minimum_refills(int mileage, const std::vector<int> &stops)
{
  int current_range { 0 }, n_refills { -1 };
  for (int i { 0 }; i < stops.size() - 1; ++i)
    if (stops[i + 1] > current_range) {
      current_range = stops[i] + mileage;
      if (stops[i + 1] > current_range)
        return -1;  // quit if distance between 2 stop is more than mileage
      n_refills += 1;
    }

  return n_refills;
}

int main()
{
  int distance {}, mileage {}, n_stops {};
  std::cin >> distance >> mileage >> n_stops;
  std::vector<int> stops(n_stops + 2, 0);
  stops[n_stops + 1] = distance;
  for (int i { 0 }; i < n_stops; ++i) std::cin >> stops[i + 1];

  std::cout << minimum_refills(mileage, stops) << '\n';
}
