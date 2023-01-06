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

    println!("{}", a + b);
}
