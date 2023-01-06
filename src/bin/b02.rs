#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    for i in a..=b {
        if 100 % i == 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
