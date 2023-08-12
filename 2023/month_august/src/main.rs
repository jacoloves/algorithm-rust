/* ↓AOJ */
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

// #[allow(unused_imports)]
// use proconio::input;
#[allow(unused_imports)]
use std::{isize, usize};

#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

use std::fmt::Debug;
use std::str::FromStr;

fn read_line<T: std::str::FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}
/* ↑AOJ */

fn main() {
    alds1_5_a();
}

fn check(k: usize, n: usize, sum: usize, as_: &Vec<usize>) -> bool {
    if sum == 0 {
        return true;
    }
    if k >= n {
        return false;
    }

    if as_[k] <= sum && check(k + 1, n, sum - as_[k], as_) {
        return true;
    }
    return check(k + 1, n, sum, as_);
}

#[allow(dead_code)]
fn alds1_5_a() {
    let n: usize = read_line()[0];
    let as_: Vec<usize> = read_line();
    let _q: usize = read_line()[0];
    let ms: Vec<usize> = read_line();

    for m in ms {
        if check(0, n, m, &as_) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
