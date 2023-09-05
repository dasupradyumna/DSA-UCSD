# Week 4

## 1. Binary Search

Self-explanatory. Time complexity **O(logn)** per query.

## 2. Binary Search with Duplicates

Same logic as a vanilla binary search; but with a variation such that, when the current element is
equal to the query, then it is returned as the answer if its preceding element is not the query (i.e
_current query is the first occurence_) or if it is the first element of the sequence, otherwise
continue searching to the left of the current query.  
Time complexity is still **O(logn)** per query.

## 3. Majority Element

Sort the sequence. Iterate through the sorted sequence once and get the count of the most frequently
occuring number. If this count is greater than half the length of the sequence, then and only then
is that element a majority element of the sequence. Else, the sequence has no majority element.  
Time complexity is **O(nlogn)**.

## 4. Improving QuickSort (with Duplicates)

Instead of the classical 2-partition, create 3-partition in each iteration where each partition
corresponds to lesser than, equal to and greater than the pivot. By creating a separate partition
for elements that are equal to pivot, we prevent the worst case time complexity from becoming
**quadratic**.  
Time complexity is **O(nlogn)**.
