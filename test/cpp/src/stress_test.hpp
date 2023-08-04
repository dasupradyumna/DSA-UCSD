#include <iostream>
#include <random>

using sz = unsigned long;

template<typename T>
class RNG {
private:
  std::mt19937 m_gen;
  std::vector<std::uniform_int_distribution<T>> m_dists;

public:
  RNG() { m_gen = std::mt19937 { std::random_device {}() }; }

  void add_dist(T lower, T upper)
  {
    m_dists.push_back(std::uniform_int_distribution<T> { lower, upper });
  }

  T from_dist(sz i = 0) { return m_dists[i](m_gen); }

  std::vector<T> create_vec(sz len, sz i = 0)
  {
    std::vector<T> sequence(len);
    for (T &v : sequence) v = this->from_dist(i);
    return sequence;
  }
};

template<typename T>
void print_vec(const std::vector<T> &vec)
{
  std::cout << '[';
  for (const T &v : vec) std::cout << v << ", ";
  std::cout << "]\n";
}
