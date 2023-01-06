#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            let tmp = k - i - j;
            if 0 < tmp && tmp <= n {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
