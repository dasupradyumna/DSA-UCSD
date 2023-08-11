# Week 3

## 1. Money Change

Straightforward calculation; one-liner solution.

## 2. Fractional Knapsack Problem

Sort both the costs and the weights in the descending order of rates (**rate = cost / weight**).
Iterate through this array, keep adding the item until capacity becomes zero. This algorithm is
greedy, and is of **O(nlogn)** time complexity.

## 3. Car Refueling

Every time the car refuels, travel the maximum possible distance and refuel at a stop only if the
next stop cannot be reached. If the distance between any two stops is more than the mileage, then
the journey is impossible. The algorithm is **linear** in time.

## 4. Maximum Advertisement Revenue

Sort both the ad prices and number of clicks, and pair them up correspondingly. This algorithm takes
**O(nlogn)** time.
