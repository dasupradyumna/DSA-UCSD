def binary_search(seq, query):
    (left, right) = (0, len(seq))
    while left < right:
        mid = left + (right - left) // 2
        if seq[mid] == query:
            return mid
        elif seq[mid] < query:
            left = mid + 1
        else:
            right = mid

    return -1


if __name__ == "__main__":
    input()  # skip length of sequence
    seq = [int(e) for e in input().split()]
    input()  # skip number of queries
    for q in input().split():
        print(binary_search(seq, int(q)), end=" ")
    print()
