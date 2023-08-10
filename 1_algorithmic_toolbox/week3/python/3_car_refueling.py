def minimum_refills(mileage, stops):
    current_range = 0
    n_refills = -1
    for i in range(len(stops) - 1):
        if stops[i + 1] > current_range:
            current_range = stops[i] + mileage
            if stops[i + 1] > current_range:
                return -1
            n_refills += 1

    return n_refills


if __name__ == "__main__":
    distance = int(input())
    mileage = int(input())
    input()  # ignore number of stops; vector length is sufficient
    stops = [0]
    stops.extend(int(i) for i in input().split())
    stops.append(distance)

    print(minimum_refills(mileage, stops))
