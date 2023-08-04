def maximum_pairwise_product_slow(sequence):
    res = -1
    for i, v1 in enumerate(sequence):
        for v2 in sequence[i + 1 :]:
            res = max(res, v1 * v2)
    return res


def maximum_pairwise_product_fast(sequence):
    m1, m2 = (0, 1) if sequence[0] >= sequence[1] else (1, 0)

    for i, v in list(enumerate(sequence))[2:]:
        if v > sequence[m1]:
            m1, m2 = i, m1
        elif v > sequence[m2]:
            m2 = i

    return sequence[m1] * sequence[m2]


def maximum_pairwise_product_fastest(sequence):
    def find_largest(begin, end):
        if begin == end:
            return [sequence[begin]]

        left = find_largest(begin, (begin + end) // 2)
        right = find_largest((begin + end) // 2 + 1, end)
        if left[0] >= right[0]:
            left.append(right[0])
            return left
        else:
            right.append(left[0])
            return right

    largest_path = find_largest(0, len(sequence) - 1)
    return largest_path[0] * max(largest_path[1:])


def main():
    _ = int(input())
    sequence = [int(n) for n in input().split()]
    print(maximum_pairwise_product_fastest(sequence))


def stress_test():
    from stress_test import Rng

    rng = Rng()
    rng.add_dist(2, 1_000)
    rng.add_dist(0, 200_000)
    n_tests = 100_000

    for t in range(n_tests):
        n = rng.from_dist(0)
        seq = rng.create_list(n, 1)

        slow = maximum_pairwise_product_slow(seq)
        fast = maximum_pairwise_product_fastest(seq)

        if fast != slow:
            print(f"\n\nTest failed!\nn = {n}\n{seq}")
            print(f"Result: (slow) {slow}, (fast) {fast}")
            break

        print(f"\rTests passed: {t+1}", end="")


if __name__ == "__main__":
    main()
    # stress_test()
