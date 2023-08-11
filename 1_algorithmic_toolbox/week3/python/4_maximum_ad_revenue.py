def maximum_ad_revenue(prices, clicks):
    prices.sort()
    clicks.sort()
    return sum(p * c for p, c in zip(prices, clicks))


if __name__ == "__main__":
    input()  # ignore first line
    prices = [int(i) for i in input().split()]
    clicks = [int(i) for i in input().split()]
    print(maximum_ad_revenue(prices, clicks))
