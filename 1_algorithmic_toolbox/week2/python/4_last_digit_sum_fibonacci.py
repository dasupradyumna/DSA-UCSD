def last_digit_sum_fibonacci(n):
    MODULO = 10

    cumulated_pisano = [0]
    a, b, sum = 1, 1, 1
    while not (a == 0 and b == 1):
        cumulated_pisano.append(sum)
        a, b = b, (a + b) % MODULO
        sum = (sum + a) % MODULO

    p_sz = len(cumulated_pisano)
    q, r = n // p_sz, n % p_sz
    return (q * cumulated_pisano[p_sz - 1] + cumulated_pisano[r]) % MODULO


if __name__ == "__main__":
    print(last_digit_sum_fibonacci(int(input())))
