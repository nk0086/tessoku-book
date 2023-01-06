#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        x: [[usize; w]; h],
        q: usize,
    }

    let mut prev = vec![vec![0; w + 5]; h + 5];
    for i in 0..h {
        for j in 0..w {
            prev[i + 1][j + 1] = x[i][j];
        }
    }

    for i in 0..h + 1 {
        for j in 0..w + 1 {
            prev[i][j + 1] += prev[i][j];
        }
    }

    for j in 0..w + 1 {
        for i in 0..h + 1 {
            prev[i + 1][j] += prev[i][j];
        }
    }

    for _ in 0..q {
        input! {
            a: usize, b: usize, c: usize, d: usize,
        }

        let count = prev[c][d] + prev[a - 1][b - 1] - prev[a - 1][d] - prev[c][b - 1];
        println!("{}", count);
    }
}
