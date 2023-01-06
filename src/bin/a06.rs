#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
    }

    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + a[i];
    }

    for _ in 0..q {
        input! {
            l: usize, r: usize,
        }

        println!("{}", prefix_sum[r] - prefix_sum[l - 1]);
    }
}
