use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::env::args;
use std::iter;

/// Prints the first several Hamming numbers using a <tt>BinaryHeap</tt>.
fn main() {
    let n: usize = args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .expect("error: limit required");
    println!("{:?}", hamming_iter().take(n).collect::<Vec<_>>());
}

fn hamming_iter() -> impl Iterator<Item = usize> {
    let mut q = BinaryHeap::new();
    q.push(Reverse(1));

    iter::from_fn(move || {
        let Reverse(i) = q.pop().unwrap();
        // remove dups
        loop {
            match q.peek() {
                Some(Reverse(p)) if *p == i => q.pop(),
                _ => break,
            };
        }
        q.push(Reverse(2 * i));
        q.push(Reverse(3 * i));
        q.push(Reverse(5 * i));
        Some(i)
    })
}

#[test]
fn hamming_iter_test() {
    assert_eq!(hamming_iter().take(0).collect::<Vec<_>>(), []);
    assert_eq!(hamming_iter().take(1).collect::<Vec<_>>(), [1]);
    assert_eq!(
        hamming_iter().take(10).collect::<Vec<_>>(),
        [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
    );
}
