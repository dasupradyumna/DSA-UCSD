MODULO = 10


def last_digit_sum_squares_fibonacci(n):
    global MODULO

    pisano10 = [0]
    a, b = 1, 1
    while not (a == 0 and b == 1):
        pisano10.append(a)
        a, b = b, (a + b) % MODULO

    p_sz = len(pisano10)
    return (pisano10[n % p_sz] * pisano10[(n + 1) % p_sz]) % MODULO


if __name__ == "__main__":
    print(last_digit_sum_squares_fibonacci(int(input())))
