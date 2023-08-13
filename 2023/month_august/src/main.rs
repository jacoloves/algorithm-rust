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
    alds1_5_c();
}

fn koch(x1: f64, y1: f64, x2: f64, y2: f64, n: usize) {
    if n == 0 {
        println!("{} {}", x1, y1);
    } else {
        let x3: f64 = (x1 * 2.0 + x2) / 3.0;
        let y3: f64 = (y1 * 2.0 + y2) / 3.0;
        let x4: f64 = x1 + (x2 - x1) / 2.0 - 3_f64.sqrt() / 6.0 * (y2 - y1);
        let y4: f64 = y1 + (y2 - y1) / 2.0 + 3_f64.sqrt() / 6.0 * (x2 - x1);
        let x5: f64 = (x1 + x2 * 2.0) / 3.0;
        let y5: f64 = (y1 + y2 * 2.0) / 3.0;
        koch(x1, y1, x3, y3, n - 1);
        koch(x3, y3, x4, y4, n - 1);
        koch(x4, y4, x5, y5, n - 1);
        koch(x5, y5, x2, y2, n - 1);
    }
}

fn alds1_5_c() {
    let n: usize = read_line()[0];
    let x1: f64 = 0.0;
    let y1: f64 = 0.0;
    let x2: f64 = 100.0;
    let y2: f64 = 0.0;
    koch(x1, y1, x2, y2, n);
    println!("{} {}", x2, y2);
}

fn marge(as_: &mut Vec<usize>, left: usize, mid: usize, right: usize) -> usize {
    let mut cnt: usize = 0;
    let n1: usize = mid - left;
    let n2: usize = right - mid;
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();
    for i in 0..n1 {
        l.push(as_[left + i]);
    }
    for i in 0..n2 {
        r.push(as_[mid + i]);
    }
    l.push(usize::MAX);
    r.push(usize::MAX);
    let mut i: usize = 0;
    let mut j: usize = 0;
    for k in left..right {
        cnt += 1;
        if l[i] <= r[j] {
            as_[k] = l[i];
            i += 1;
        } else {
            as_[k] = r[j];
            j += 1;
        }
    }
    return cnt;
}

fn marge_sort(as_: &mut Vec<usize>, left: usize, right: usize) -> usize {
    let mut cnt: usize = 0;
    if left + 1 < right {
        let mid: usize = (left + right) / 2;
        cnt += marge_sort(as_, left, mid);
        cnt += marge_sort(as_, mid, right);
        cnt += marge(as_, left, mid, right);
    }

    return cnt;
}

#[allow(dead_code)]
fn alds1_5_b() {
    let n: usize = read_line()[0];
    let mut as_: Vec<usize> = read_line();

    let cnt = marge_sort(&mut as_, 0, n);

    println!(
        "{}",
        as_.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    println!("{}", cnt);
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
