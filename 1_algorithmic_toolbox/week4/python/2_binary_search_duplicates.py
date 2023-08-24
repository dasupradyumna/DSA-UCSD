def binary_search_duplicates(seq, query):
    (left, right) = (0, len(seq))
    while left < right:
        mid = left + (right - left) // 2
        if seq[mid] == query and (mid == 0 or seq[mid - 1] != query):
            return mid
        elif seq[mid] >= query:
            right = mid
        else:
            left = mid + 1

    return -1


if __name__ == "__main__":
    input()  # skip length of sequence
    seq = [int(e) for e in input().split()]
    input()  # skip number of queries
    for q in input().split():
        print(binary_search_duplicates(seq, int(q)), end=" ")
    print()
