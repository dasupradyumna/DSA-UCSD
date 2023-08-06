# Week 2

## 1. Nth Fibonacci Number

Calculate upwards from first numbers of the sequence using the formula definition. Linear time
complexity **O(n)**.

## 2. Last Digit of Fibonacci Number

Same as above approach, except in each iteration, only the units digit is preserved after addition.
(i.e _modulo 10_)

## 3. Huge Fibonacci Number (Modulo M)

The sequence created from the Fibonacci sequence modulo a positive integer (**>1**) is a periodic
sequence with a period called the **Pisano Period**. Given a large number **N**, **F<sub>N</sub> mod
M** can be calculated by first generating the **Pisano cycle** and then transforming **N** to an
index into the generated cycle.

## 4. Last Digit of Sum of Fibonacci Numbers

The sequence of last digits of Fibonacci numbers is simply the **modulo-10** sequence from above.
Since the digits are (Pisano) periodic, every period will contribute the same accumulated digit.
Using this, we can calculate how many full pisano periods and a final partial period that **N**
contains. Full periods contribute the above accumulated digit, and partial period's contribution can
be calculated from the **Pisano** cycle.

## 5. Last Digit of Partial Sum of Fibonacci Numbers

Use the above function (which calculates last digit of **S<sub>N**, sum of first **N** Fibonacci
numbers) to calculate the last digit of sum of Fibonacci numbers from **M** to **N** (inclusive).  
**S<sub>MN** = **(S<sub>N</sub> - S<sub>M-1</sub>) mod 10**

## 6. Last Digit of Sum of Squares of Fibonacci Numbers

Using the classical visualization of stacked squares for drawing the **Fibonacci spiral**, we can
derive a formula for the sum of squares of Fibonacci numbers. Since the square of a Fibonacci number
is the area of its corresponding geometrical square, the sum of squares will be the total area of
stacked squares structure.  
From this observation, we arrive at the formula : **SS<sub>N</sub> = F<sub>N</sub> x F <sub>N+1**

## 7. GCD of 2 Integers

Classic Euclidean Algorithm.
