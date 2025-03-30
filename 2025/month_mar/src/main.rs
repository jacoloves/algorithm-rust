#[allow(unused_imports)]
use itertools::min;
#[allow(unused_imports)]
use proconio::input;
/* ↓aoj */
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

use std::iter::zip;
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

#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
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
    abc310a();
}

#[allow(dead_code)]
fn abc310a() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        mut d: [usize; n]
    }

    for elem in d.iter_mut() {
        *elem += q;
    }

    let min_d = d.iter().min().unwrap();

    if p >= *min_d {
        println!("{}", *min_d);
    } else {
        println!("{}", p);
    }
}

#[allow(dead_code)]
fn abc311a() {
    input! {
        _: usize,
        s: String
    }

    let mut a_cnt = 0;
    let mut b_cnt = 0;
    let mut c_cnt = 0;

    let mut idx = 0;

    for c in s.chars() {
        idx += 1;
        if c == 'A' && a_cnt == 0 {
            a_cnt = 1;
        }
        if c == 'B' && b_cnt == 0 {
            b_cnt = 1;
        }
        if c == 'C' && c_cnt == 0 {
            c_cnt = 1;
        }

        if a_cnt == 1 && b_cnt == 1 && c_cnt == 1 {
            break;
        }
    }

    println!("{}", idx);
}

#[allow(dead_code)]
fn abc314a() {
    input! {
        n: usize
    }

    let pi100: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";

    let s1 = &pi100[0..n + 2];

    println!("{}", s1);
}

#[allow(dead_code)]
fn abc317a() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        mut p: [usize; n]
    }

    let sub_a = x - h;
    let mut ans: usize = 0;

    for (pa, num) in zip(p.iter_mut(), 1..) {
        if *pa >= sub_a {
            ans = num;
            break;
        }
    }

    println!("{}", ans);
}
