if __name__ == "__main__":
    from functools import cmp_to_key

    input()  # skip length of sequence input
    sequence = [i.strip() for i in input().split()]
    sequence.sort(
        key=cmp_to_key(
            lambda s1, s2: 1 if s2 + s1 > s1 + s2 else -1 if s2 + s1 < s1 + s2 else 0
        )
    )
    print("".join(sequence))
