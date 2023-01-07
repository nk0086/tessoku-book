#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        d: usize,
    }

    let mut prev_left = vec![0; n + 1];
    let mut prev_right = vec![0; n + 1];

    for i in 1..=n {
        prev_left[i] = prev_left[i - 1].max(a[i - 1]);
        prev_right[i] = prev_right[i - 1].max(a[n - i]);
    }

    prev_right.reverse();

    for _ in 0..d {
        input! {
            l: usize, r: usize,
        }

        let ans = prev_left[l - 1].max(prev_right[r]);
        println!("{}", ans);
    }
}
