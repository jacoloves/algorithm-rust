#[allow(unused_imports)]
use proconio::input;

/* ↓aoj */
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

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

#[allow(dead_code)]
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
    algo362();
}

#[allow(dead_code)]
fn algo362() {
    let mut n: i32 = read_line()[0];

    let mut cnt = 0;
    while n > 0 {
        if n % 2 == 0 {
            n /= 2;
            cnt += 1;
        } else {
            n -= 1;
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn algo359() {
    let n: usize = read_line()[0];

    let ans = n / 5;
    let div_n = n % 5;

    println!("{}", ans + div_n);
}
