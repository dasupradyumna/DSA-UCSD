MODULO = 10


def last_digit_sum(n, cp):
    global MODULO

    p_sz = len(cp)
    q, r = n // p_sz, n % p_sz
    return (q * cp[p_sz - 1] + cp[r]) % MODULO


def last_digit_partial_sum_fibonacci(m, n):
    global MODULO

    cp = [0]  # cumulated pisano modulos
    a, b, sum = 1, 1, 1
    while not (a == 0 and b == 1):
        cp.append(sum)
        a, b = b, (a + b) % MODULO
        sum = (sum + a) % MODULO

    return (
        last_digit_sum(n, cp)
        if m == 0
        else (last_digit_sum(n, cp) - last_digit_sum(m - 1, cp) + MODULO) % MODULO
    )


if __name__ == "__main__":
    print(last_digit_partial_sum_fibonacci(*[int(i) for i in input().split()]))
