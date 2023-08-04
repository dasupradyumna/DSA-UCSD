def fibonacci(n):
    if n <= 1:
        return n

    a, b = 0, 1
    for _ in range(n - 1):
        a, b = b, (a + b) % 10
    return b


if __name__ == "__main__":
    print(fibonacci(int(input())))
