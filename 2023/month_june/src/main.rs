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
    abc097_b();
}

#[allow(dead_code)]
fn abc097_b() {
    input! {
        x: usize,
    }

    let mut ans = 0;

    for i in 1..=x {
        for j in 2..=1000 {
            let t = i.pow(j as u32);
            if x >= t {
                ans = cmp::max(ans, t);
            } else {
                break;
            }
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc042_b() {
    input! {
        n: usize,
        _l: usize,
        mut s: [String; n],
    }

    s.sort();

    let mut ans: String = "".to_string();

    for e in s.iter() {
        ans += e;
    }

    println!("{}", ans);
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
