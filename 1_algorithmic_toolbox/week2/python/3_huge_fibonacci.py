def fibonacci_mod_m(n, m):
    pisano_period = [0]
    a, b = 1, 1
    while not (a == 0 and b == 1):
        pisano_period.append(a)
        a, b = b, (a + b) % m

    return pisano_period[n % len(pisano_period)]


if __name__ == "__main__":
    print(fibonacci_mod_m(*[int(i) for i in input().split()]))
