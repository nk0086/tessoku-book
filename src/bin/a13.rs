#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }

    let mut count = 0;
    for i in 0..n - 1 {
        count += search(&a, i, k);
    }

    println!("{}", count);
}

fn search(a: &Vec<usize>, i: usize, k: usize) -> usize {
    let mut ng = a.len() as i64;
    let mut ok = i as i64;

    while (ng - ok) > 1 {
        let mid = (ok + ng) / 2;

        if a[mid as usize] - a[i] > k {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    (ok - i as i64) as usize
}
