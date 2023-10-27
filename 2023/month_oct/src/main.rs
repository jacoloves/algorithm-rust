use itertools::concat;
#[allow(unused_imports)]
use proconio::input;
use proconio::source::Readable;

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
/* ↑aoj */

fn main() {
    abc323_a();
}

#[allow(dead_code)]
fn abc323_a() {
    input! {
        s: String,
    }

    for (i, c) in s.chars().enumerate() {
        if i % 2 == 1 && c != '0' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

#[allow(dead_code)]
fn abc325_b_ans() {
    input! {
        n: usize,
        w: [(i64, i64); n],
    }

    let mut res = 0;

    for t in 0..24 {
        let mut num = 0;

        for i in 0..n {
            let real_t = (t + w[i].1) % 24;

            if real_t >= 9 && real_t < 18 {
                num += w[i].0;
            }
        }

        res = cmp::max(res, num);
    }

    println!("{}", res);
}

#[allow(dead_code)]
fn abc325_b() {
    input! {
        n: usize,
        mut v: [(usize, usize); n],
    }

    v.sort_by(|a, b| a.1.cmp(&b.1));

    let mut ans: usize = 0;
    for i in 0..n {
        let time = v[i].1 + 8;
        let mut tmp_ans: usize = v[i].0;

        for j in i + 1..n {
            if v[j].1 <= time {
                tmp_ans += v[j].0;
            } else {
                break;
            }
        }

        ans = cmp::max(ans, tmp_ans);
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc325_a() {
    input! {
        s: String,
        mut t: String,
    }

    t = "san".to_string();

    println!("{} {}", s, t);
}