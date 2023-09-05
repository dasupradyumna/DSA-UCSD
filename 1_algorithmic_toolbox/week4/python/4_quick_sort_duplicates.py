def swap(seq, a, b):
    seq[a], seq[b] = seq[b], seq[a]


def pivot(seq, lo, hi):
    def median(a, b, c):
        nonlocal seq
        if seq[b] <= seq[a] and seq[a] <= seq[c]:
            return a
        elif seq[a] <= seq[b] and seq[b] <= seq[c]:
            return b
        else:
            return c

    d = hi - lo
    p = lo + d // 2
    if d >= 50:
        if d >= 100:
            lo = median(lo, lo + d // 8, lo + d // 4)
            p = median(p - d // 8, p, p + d // 8)
            hi = median(hi - d // 4, hi - d // 8, hi)
        p = median(lo, p, hi)

    return p


def part3way(seq, lo, hi):
    swap(seq, lo, pivot(seq, lo, hi))
    p = seq[lo]

    iter, lesser, greater = lo + 1, lo + 1, hi - 1
    while iter <= greater:
        if seq[iter] > p:
            swap(seq, iter, greater)
            greater -= 1
        elif seq[iter] == p:
            iter += 1
        else:
            swap(seq, iter, lesser)
            iter += 1
            lesser += 1

    swap(seq, lo, lesser - 1)
    return (lesser - 1, greater + 1)


def quick_sort(seq):
    stack = [(0, len(seq))]
    while len(stack) != 0:
        lo, hi = stack.pop()
        if hi - lo <= 1:
            continue

        p1, p2 = part3way(seq, lo, hi)
        stack.append((lo, p1))
        stack.append((p2, hi))


if __name__ == "__main__":
    input()  # skip length of the sequence
    seq = [int(i) for i in input().split()]
    quick_sort(seq)

    for e in seq:
        print(e, end=" ")
    print()
