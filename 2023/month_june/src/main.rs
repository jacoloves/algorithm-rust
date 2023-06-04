#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(unused_imports)]
use std::cmp;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]

fn main() {
    sumitb2019_c();
}

#[allow(dead_code)]
fn sumitb2019_c() {
    input! {
        x: usize,
    }

    let mut dp = vec![false; 101010];
    dp[0] = true;

    for i in 0..x {
        if dp[i] {
            for d in 0..6 {
                dp[i + d + 100] = true;
            }
        }
    }

    if dp[x] {
        println!("1");
    } else {
        println!("0");
    }
}
