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

## 5. Collecting Signaturees

### Check pairwise

Select a time period, find all time periods that overlap with the selected time period and pick the
smallest ending time of all the overlapping time periods. Remove all overlapping time periods then.
Repeat this process and populate a list with the smallest ending time in every such iteration until
no time period is left.  
Worst case time complexity: **O(n<sup>2</sup>)**

### Sort by ending time

Sort all the time periods by the ending time. Pick the ending time of the first time period as the
first visit time. Then, start iterating through the sorted list of time periods until the starting
time of the current time period is greater than the current overlap's ending time. Update the
current overlap and repeat the above process until the entire list is exhausted.  
Worst case time complexity: **O(nlogn)**

## 6. Maximum Number of Prizes

Get the maximum integer **a** such that sum of all integers from 1 to **a** does not exceed **n**.

## 7. Largest Concatenation

Simply sort the list of integers using a custom comparator function; where given 2 integers **a**
and **b**, **a** is sorted to the left of **b** if the concatenated number **ab** is greater than or
equal to **ba**, and to the right otherwise. The integers in the sorted list can simply be
concatenated together for the final number.  
Time complexity is **O(nlogn)**.
