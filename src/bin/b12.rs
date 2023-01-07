#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ok = 100.;
    let mut ng = 0.;
    while (ok - ng) > 1e-3 {
        let mid = (ok + ng) / 2.;
        let fx = mid * mid * mid + mid;

        if fx >= n as f64 {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
