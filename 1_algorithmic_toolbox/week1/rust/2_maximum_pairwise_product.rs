use std::cmp::Ordering;
use std::{cmp, io, iter};

fn main() {
    let mut input = String::new();
    read_lines(&mut input, 2);
    let mut input = input.split_whitespace().map(|i| i.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let sequence: Vec<_> = input.take(n as usize).collect();

    println!("{}", maximum_pairwise_product_fastest(&sequence));
}

fn read_lines(input: &mut String, n_lines: i32) {
    for _ in 0..n_lines {
        io::stdin().read_line(input).unwrap();
    }
}

/* STRESS_TEST
fn main() {
    let mut rng = rand::thread_rng();
    let mut rng = stress_test::RNG::<i32>::new(&mut rng);
    rng.add_dist(2, 1_000);
    rng.add_dist(0, 200_000);
    let n_tests = 1e5 as u32;

    for t in 1..=n_tests {
        let n = rng.from_dist(0) as usize;
        let seq = rng.create_vec(n, 1);

        let slow = maximum_pairwise_product_slow(&seq);
        let fast = maximum_pairwise_product_fastest(&seq);

        if fast != slow {
            println!("\n\nTest failed!\nn = {n}\n{seq:?}");
            println!("Result: (slow) {slow}, (fast) {fast}");
            break;
        }
        print!("\r Tests passed: {t}");
    }
}
*/

fn maximum_pairwise_product_slow(sequence: &Vec<i32>) -> i64 {
    sequence
        .iter()
        .enumerate()
        .flat_map(|(i, v)| sequence.iter().skip(i + 1).zip(iter::repeat(v)))
        .map(|(a, b)| (*a as i64, *b as i64))
        .fold(-1, |res, (v1, v2)| cmp::max(res, v1 * v2))
}

fn maximum_pairwise_product_fast(sequence: &Vec<i32>) -> i64 {
    let (mut max1, mut max2) = match sequence[0].cmp(&sequence[1]) {
        Ordering::Greater | Ordering::Equal => (0, 1),
        Ordering::Less => (1, 0),
    };

    sequence.iter().enumerate().skip(2).for_each(|(idx, val)| {
        if *val > sequence[max1] {
            max2 = max1;
            max1 = idx;
        } else if *val > sequence[max2] {
            max2 = idx;
        }
    });

    sequence[max1] as i64 * sequence[max2] as i64
}

fn maximum_pairwise_product_fastest(sequence: &Vec<i32>) -> i64 {
    fn find_largest(seq: &Vec<i32>, begin: usize, end: usize) -> Vec<i32> {
        if begin == end {
            return vec![seq[begin]];
        }

        let mut left = find_largest(seq, begin, (begin + end) / 2);
        let mut right = find_largest(seq, (begin + end) / 2 + 1, end);
        match left[0].cmp(&right[0]) {
            Ordering::Greater | Ordering::Equal => {
                left.push(right[0]);
                left
            }
            Ordering::Less => {
                right.push(left[0]);
                right
            }
        }
    }

    let mut largest_path = find_largest(sequence, 0, sequence.len() - 1).into_iter();
    let largest = largest_path.next().unwrap() as i64;
    let second_largest = largest_path.fold(-1, |res, v| cmp::max(res, v)) as i64;

    largest * second_largest
}
