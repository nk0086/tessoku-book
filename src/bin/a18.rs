#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, s: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..s + 1 {
            if dp[i][j] == 0 {
                continue;
            }

            dp[i + 1][j] = 1;
            if j + a[i] <= s {
                dp[i + 1][j + a[i]] = 1;
            }
        }
    }

    println!("{}", if dp[n][s] == 1 { "Yes" } else { "No" });
}
