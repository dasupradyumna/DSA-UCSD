def maximum_cost_capacity(capacity, items):
    items.sort(key=lambda item: item[0], reverse=True)

    total = 0
    for rate, weight in items:
        if capacity <= 0:
            break

        total += rate * min(weight, capacity)
        capacity -= weight

    return total


if __name__ == "__main__":
    n, capacity = [int(i) for i in input().split()]
    items = []
    for _ in range(n):
        rate, weight = (int(i) for i in input().split())
        if weight > 0:
            rate /= weight
        items.append((rate, weight))

    print(f"{maximum_cost_capacity(capacity, items):.4f}")
