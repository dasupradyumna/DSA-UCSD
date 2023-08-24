# Week 4

## 1. Binary Search

Self-explanatory. Time complexity **O(logn)** per query.

## 2. Binary Search with Duplicates

Same logic as a vanilla binary search; but with a variation such that, when the current element is
equal to the query, then it is returned as the answer if its preceding element is not the query (i.e
_current query is the first occurence_) or if it is the first element of the sequence, otherwise
continue searching to the left of the current query.  
Time complexity is still **O(logn)** per query.
