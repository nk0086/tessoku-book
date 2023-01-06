#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut prev = vec![0; t + 2];
    let mut work = vec![0; t + 2];
    for i in 0..n {
        let (l, r) = lr[i];
        work[l] += 1;
        work[r] -= 1;
    }

    for i in 0..t {
        prev[i + 1] = prev[i] + work[i];
    }

    dbg!(&prev);
    for i in 0..t {
        println!("{}", prev[i + 1]);
    }
}
