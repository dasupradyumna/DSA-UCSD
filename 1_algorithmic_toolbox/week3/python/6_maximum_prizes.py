if __name__ == "__main__":
    n = int(input())
    a = 1
    while n >= a:
        n -= a
        a += 1
    a -= 1

    print(a)
    for i in range(1, a):
        print(i, end=" ")
    print(a + n)
