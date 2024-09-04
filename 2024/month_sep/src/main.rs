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
    abc367_b();
}

#[allow(dead_code)]
fn abc367_b() {
    input! {
        x: f32,
    }

    let mut multi_x = x * 1000.0;

    let mut divid_num = 1000.0;
    for _ in 1..=3 {
        if multi_x % 10.0 == 0.0 {
           divid_num /= 10.0; 
        } else {
            break;
        }
        multi_x /= 10.0;
    } 

    let ans = multi_x / divid_num;

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc367_a() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if b < c {
        if b < a && a < c {
            println!("No");
        } else {
            println!("Yes");
        }
        return;
    }

    if c < a && a < b {
        println!("Yes");
    } else {
        println!("No");
    }
}
