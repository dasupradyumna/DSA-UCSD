def fibonacci(n):
    if n <= 1:
        return n

    a, b = 1, 0
    for _ in range(n - 1):
        a = a + b
        b = a - b
    return a


if __name__ == "__main__":
    print(fibonacci(int(input())))
