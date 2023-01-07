#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize
    }

    a.sort();
    for _ in 0..q {
        input! {
            x: usize
        }

        let ans = search(&a, x);
        println!("{}", ans);
    }
}

fn search(a: &Vec<usize>, x: usize) -> usize {
    let mut ok = a.len() as i64;
    let mut ng = -1;

    while (ok - ng) > 1 {
        let mid = (ok + ng) / 2;

        if a[mid as usize] < x {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    ok as usize
}
