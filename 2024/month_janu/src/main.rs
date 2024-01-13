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
    abc334_b();
}

#[allow(dead_code)]
fn abc334_floor(a: isize, b: isize) -> isize {
    let c = (a % b + b) % b;
    (a - c) / b
}

#[allow(dead_code)]
fn abc334_b() {
    input! {
        a: isize,
        m: isize,
        mut l: isize,
        mut r: isize,
    }

    l -= a;
    r -= a;

    println!("{}", abc334_floor(r, m) - abc334_floor(l - 1, m));
}

#[allow(dead_code)]
fn abc334_a() {
    input! {
        b: usize,
        g: usize,
    }

    if b > g {
        println!("Bat");
    } else {
        println!("Glove");
    }
}

#[allow(dead_code)]
fn abc335_b() {
    input! {
        n: usize,
    }

    for x in 0..=n {
        for y in 0..=n {
            for z in 0..=n {
                if x + y + z <= n {
                    println!("{} {} {}", x, y, z);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn abc335_a() {
    input! {
        s: String,
    }

    let result = if s.ends_with("2023") {
        let prefix = &s[0..s.len() - 4];
        format!("{}{}", prefix, "2024")
    } else {
        s
    };

    println!("{}", result);
}
