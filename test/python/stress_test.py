import random


class Rng:
    def __init__(self):
        self.dists = []

    def add_dist(self, lower, upper):
        self.dists.append((lower, upper))

    def from_dist(self, i):
        return random.randint(*self.dists[i])

    def create_list(self, len, i):
        return [self.from_dist(i) for _ in range(len)]
