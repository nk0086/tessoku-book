#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![-1; w + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..w + 1 {
            if dp[i][j] == -1 {
                continue;
            }

            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            if j + wv[i].0 <= w {
                dp[i + 1][j + wv[i].0] = dp[i + 1][j + wv[i].0].max(dp[i][j] + wv[i].1 as i64);
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
