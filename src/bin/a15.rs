#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

use std::collections::HashMap;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let aa = a.clone().into_iter().collect::<HashSet<usize>>();
    let mut aa = aa.into_iter().collect::<Vec<usize>>();
    aa.sort();

    let mut map = HashMap::new();
    for (i, ai) in aa.iter().enumerate() {
        map.insert(ai, i);
    }

    let mut ans = vec![];
    for ai in a {
        ans.push(map[&ai] + 1);
    }

    println!(
        "{}",
        ans.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
