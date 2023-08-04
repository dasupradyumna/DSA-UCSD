# Week 1

## 1. Sum of 2 Digits

Self-explanatory.

## 2. Maximum Pairwise Product

### Slow Naive method

Iterate through all possible pairs of integers and find the maximum calculated product. The time
complexity for this method is quadratic **O(n<sup>2</sup>)**.

### Fast Efficient method

Since the maximum pairwise product can only be achieved using the 2 largest elements, we just need
to find these 2 integers. This is under the assumption that all elements are non-negative integers.

1. **O(1.5n)**  
Iterate the array once and keep track of the two largest integers encountered simultaneously.
2. **O(n+logn)**  
Split the array in half recursively and cache all the (**n-1**) comparisons done for the largest
element. Using this cached path (**logn** length), the second largest element can be found.  
[_More information_](https://stackoverflow.com/questions/9889679/find-second-largest-number-in-array-at-most-nlog%E2%82%82n%E2%88%922-comparisons)
