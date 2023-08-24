def majority_element(seq):
    n_maj, curr, n_curr = -1, -1, -1

    seq.sort()
    for e in seq:
        if e != curr:
            n_maj = max(n_maj, n_curr)
            curr, n_curr = e, 0
        n_curr += 1

    return 1 if max(n_maj, n_curr) > len(seq) // 2 else 0


if __name__ == "__main__":
    input()  # skip the length of sequence
    seq = [int(i) for i in input().split()]
    print(majority_element(seq))
