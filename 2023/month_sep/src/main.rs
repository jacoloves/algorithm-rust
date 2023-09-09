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
    algo364();
}

#[allow(dead_code)]
fn algo364() {
    let mut n: usize = read_line()[0];

    let mut cnt: usize = 0;

    loop {
        if n == 0 {
            break;
        }

        cnt += 1;

        if n % 3 == 0 {
            n /= 3;
        } else {
            n -= 1;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn algo361() {
    let line: Vec<i32> = read_line();
    let n: i32 = line[0];
    let m: i32 = line[1];

    let mut a: Vec<i32> = read_line();
    (&mut a).sort();
    let b: Vec<i32> = read_line();

    let mut cnt = 0;
    let mut ans = 0;
    for i in 0..n {
        for j in cnt..m {
            if m == j {
                break;
            }

            if a[i as usize] <= b[j as usize] {
                ans += 1;
                cnt += 1;
                break;
            } else {
                cnt += 1;
            }
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn algo360() {
    let mut x: i32 = read_line()[0];
    let a: Vec<i32> = read_line();
    let mut a1: i32 = a[0];
    let mut a2: i32 = a[1];
    let mut a3: i32 = a[2];
    let mut a4: i32 = a[3];

    let mut cnt = 0;

    while x > 0 {
        if a1 > 0 && x >= 50 {
            x -= 50;
            a1 -= 1;
            cnt += 1;
        } else if a2 > 0 && x >= 10 {
            x -= 10;
            a2 -= 1;
            cnt += 1;
        } else if a3 > 0 && x >= 5 {
            x -= 5;
            a3 -= 1;
            cnt += 1;
        } else if a4 > 0 && x >= 1 {
            x -= 1;
            a4 -= 1;
            cnt += 1;
        } else {
            break;
        }
    }

    println!("{}", cnt);
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
