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
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }

    let mut ab = vec![];
    let mut cd = vec![];

    for i in 0..n {
        for j in 0..n {
            ab.push(a[i] + b[j]);
            cd.push(c[i] + d[j]);
        }
    }

    ab.sort();
    cd.sort();

    for i in 0..ab.len() {
        let abi = ab[i];
        let mut ok = 0;
        let mut ng = ab.len();
        while (ng - ok) > 1 {
            let mid = (ok + ng) / 2;

            if abi + cd[mid] > k {
                ng = mid;
            } else {
                ok = mid;
            }
        }

        if abi + cd[ok] == k {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
