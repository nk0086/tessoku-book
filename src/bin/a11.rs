#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut ok = 0;
    let mut ng = n;
    while (ng - ok) > 1 {
        let mid = (ok + ng) / 2;

        if a[mid] <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok + 1);
}
