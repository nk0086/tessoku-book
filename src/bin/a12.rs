#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ok = 1_000_000_000 + 10;
    let mut ng = 0;

    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        let mut count = 0;

        for ai in a.iter() {
            count += mid / ai;
        }

        if count >= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
