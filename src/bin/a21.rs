#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    }

    let mut p = vec![0; n + 2];
    let mut a = vec![0; n + 2];

    for i in 0..n {
        p[i + 1] = pa[i].0;
        a[i + 1] = pa[i].1;
    }

    let mut dp = vec![vec![0; 2000 + 10]; 2000 + 10];
    for len in (0..=n - 2).rev() {
        //外側から決める必要がある。
        for l in 1..=n - len {
            let r = l + len;
            let mut score1 = 0;
            let mut score2 = 0;

            if l <= p[l - 1] && p[l - 1] <= r {
                score1 = a[l - 1];
            }

            if l <= p[r + 1] && p[r + 1] <= r {
                score2 = a[r + 1];
            }

            if l == 1 {
                dp[l][r] = dp[l][r + 1] + score2;
            } else if r == n {
                dp[l][r] = dp[l - 1][r] + score1;
            } else {
                dp[l][r] = (dp[l - 1][r] + score1).max(dp[l][r + 1] + score2);
            }
        }
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        ans = ans.max(dp[i][i]);
    }

    println!("{}", ans);
}
