class Period:
    def __init__(self, start, end):
        self.start = start
        self.end = end

    def __repr__(self):
        return f"Period({self.start},{self.end})"


def minimum_visits_fast(periods):
    periods.sort(key=lambda p: p.end)

    current_end = periods[0].end
    visits = [current_end]
    for p in periods[1:]:
        if p.start > current_end:
            current_end = p.end
            visits.append(current_end)

    return visits


def main():
    n = int(input())
    periods = [Period(*input().split()) for _ in range(n)]
    visits = minimum_visits_fast(periods)
    print(len(visits))
    for v in visits:
        print(v, end=" ")
    print()


#################################### STRESS TESTING ####################################


def minimum_visits_slow(periods):
    overlap = None
    current = None
    periods.sort(key=lambda p: p.end, reverse=True)

    def retain_period(period):
        nonlocal current, overlap
        remove = current.start <= period.end and current.end >= period.start
        if remove:
            overlap = min(overlap, period.end)
        return not remove

    visits = []
    while len(periods) > 0:
        current = periods.pop()
        overlap = current.end
        periods = list(filter(retain_period, periods))
        visits.append(overlap)

    visits.sort()
    return visits


def stress_test():
    from stress_test import Rng

    rng = Rng()
    rng.add_dist(1, 10)
    rng.add_dist(0, 20)
    n_tests = 100_000

    for t in range(n_tests):
        n = rng.from_dist(0)
        periods = []
        for _ in range(n):
            start, end = rng.from_dist(1), rng.from_dist(1)
            if start > end:
                start, end = end, start
            periods.append(Period(start, end))

        slow = minimum_visits_slow(periods.copy())
        fast = minimum_visits_fast(periods.copy())

        if fast != slow:
            print(f"\n\nTest failed!\nn = {n}\n{periods}")
            print(f"Result:\n (slow) {slow}\n (fast) {fast}")
            break

        print(f"\rTests passed: {t+1}", end="")


if __name__ == "__main__":
    main()
    # stress_test()
