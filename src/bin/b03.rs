#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for a_vec in a.iter().combinations(3) {
        // convert a_vec to tuple
        let (i, j, k) = (a_vec[0], a_vec[1], a_vec[2]);
        if i + j + k == 1000 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
