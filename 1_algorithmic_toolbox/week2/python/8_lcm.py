def lcm(a, b):
    lcm = a * b
    while b > 0:
        a, b = b, a % b
    return lcm // a


if __name__ == "__main__":
    print(lcm(*[int(i) for i in input().split()]))
