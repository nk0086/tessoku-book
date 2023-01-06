#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize, x: usize,
        a: [usize; n],
    }

    if a.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
