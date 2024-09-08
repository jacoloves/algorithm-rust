#[allow(unused_imports)]
use itertools::min;
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::Usize1;
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
    abc370_b();
}

#[allow(dead_code)]
fn abc370_b() {
    input! {
        n: usize,
    }

    let mut a = vec![];

    for i in 0..n {
        input! {
            r: [usize; i + 1]
        }
        a.push(r);
    }

    let mut b: usize = 0;
    for c in 0..n {
        b = if b >= c { a[b][c] - 1 } else { a[c][b] - 1 };
    }

    let ans = b + 1;
    println!("{}", ans);
}

#[allow(dead_code)]
fn abc370_a() {
    input! {
        l: usize,
        r: usize,
    }

    if l == 1 && r == 0 {
        println!("Yes");
        return;
    }

    if l == 0 && r == 1 {
        println!("No");
        return;
    }

    println!("Invalid");
}

#[allow(dead_code)]
fn abc368_b() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut cnt = 0;
    loop {
        cnt += 1;
        a.sort_by(|a, b| b.partial_cmp(a).unwrap());
        a[0] -= 1;
        a[1] -= 1;

        let mut zero_cnt = 0;
        for e in &a {
            if *e == 0 {
                zero_cnt += 1;
            }
        }

        if zero_cnt >= n - 1 {
            break;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn abc368_a() {
    input! {
        k: usize,
        n: usize,
        mut a: [usize; k],
    }

    let mut back_a: Vec<usize> = vec![];

    for _ in 1..=n {
        let tmp_n = a.pop().unwrap();
        back_a.push(tmp_n);
    }

    back_a.reverse();

    for e in back_a {
        print!("{} ", e);
    }

    for e in a {
        print!("{} ", e);
    }

    println!();
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
