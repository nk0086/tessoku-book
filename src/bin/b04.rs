#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        mut n: Chars,
    }

    n.reverse();
    let mut n_10 = 0;
    for i in 0..n.len() {
        n_10 += (n[i] as i32 - '0' as i32) * 2_i32.pow(i as u32);
    }

    println!("{}", n_10);
}
