#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    // output n as binary with 10 digits
    println!("{:010b}", n);
}
