#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    }

    let mut dp = [std::usize::MAX; 100_000 + 5];
    dp[0] = 0;
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = dp[i].min(dp[i - 1] + a[i - 1]);
        if i - 2 < n - 2 {
            dp[i] = dp[i].min(dp[i - 2] + b[i - 2]);
        }
    }

    let mut place = n;
    let mut ans = vec![];
    loop {
        ans.push(place);
        if place == 1 {
            break;
        }

        if dp[place - 2] + a[place - 2] == dp[place - 1] {
            place -= 1;
        } else {
            place -= 2;
        }
    }

    println!("{}", ans.len());
    ans.reverse();
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
