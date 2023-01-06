#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut prev = vec![0; d + 1];
    let mut attend = vec![0; d + 1];
    for i in 0..n {
        let (l, r) = lr[i];
        attend[l - 1] += 1;
        attend[r] -= 1;
    }

    dbg!(&attend);
    for i in 0..d {
        prev[i + 1] = prev[i] + attend[i];
    }

    for i in 0..d {
        println!("{}", prev[i + 1]);
    }
}
