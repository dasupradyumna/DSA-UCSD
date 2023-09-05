fn main() {
    let mut buffer = String::new();
    let mut seq = line_reader(&mut buffer, 2).skip(1).collect();
    quick_sort(&mut seq);

    seq.iter().for_each(|e| print!("{e} "));
    println!();
}

fn line_reader<'a, T>(buffer: &'a mut String, n_lines: usize) -> Box<dyn Iterator<Item = T> + 'a>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    buffer.clear();
    for _ in 0..n_lines {
        std::io::stdin().read_line(buffer).unwrap();
    }

    Box::new(buffer.split_whitespace().map(|i| i.parse().unwrap()))
}

fn quick_sort(seq: &mut Vec<i32>) {
    let mut stack = vec![(0, seq.len())];
    while !stack.is_empty() {
        let (lo, hi) = stack.pop().unwrap();
        if hi - lo <= 1 {
            continue;
        }

        let (p1, p2) = part3way(seq, lo, hi);
        stack.push((lo, p1));
        stack.push((p2, hi));
    }
}

fn part3way(seq: &mut Vec<i32>, lo: usize, hi: usize) -> (usize, usize) {
    let pivot = pivot(&seq, lo, hi - 1);
    seq.swap(lo, pivot);
    let pivot = seq[lo];

    let (mut iter, mut lesser, mut greater) = (lo + 1, lo + 1, hi - 1);
    while iter <= greater {
        match seq[iter].cmp(&pivot) {
            std::cmp::Ordering::Greater => {
                seq.swap(iter, greater);
                greater -= 1;
            },
            std::cmp::Ordering::Equal => iter += 1,
            std::cmp::Ordering::Less => {
                seq.swap(iter, lesser);
                lesser += 1;
                iter += 1;
            },
        }
    }
    seq.swap(lo, lesser - 1);

    (lesser - 1, greater + 1)
}

fn pivot(seq: &Vec<i32>, mut lo: usize, mut hi: usize) -> usize {
    let median = |a, b, c| {
        if seq[b] <= seq[a] && seq[a] <= seq[c] {
            a
        } else if seq[a] <= seq[b] && seq[b] <= seq[c] {
            b
        } else {
            c
        }
    };

    let d = hi - lo;
    let mut pivot = lo + d / 2;
    if d >= 50 {
        if d >= 100 {
            lo = median(lo, lo + d / 8, lo + d / 4);
            pivot = median(pivot - d / 8, pivot, pivot + d / 8);
            hi = median(hi - d / 4, hi - d / 8, hi);
        }
        pivot = median(lo, pivot, hi);
    }

    pivot
}
