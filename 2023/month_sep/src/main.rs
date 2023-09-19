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
    abc281_c();
}

#[allow(dead_code)]
fn abc281_c() {
    input! {
        n: usize,
        mut t: isize,
        mut a: [isize; n],
    }

    let sum_a: isize = a.iter().sum();

    let mut mus: usize = 1;
    let mut mus_t: isize = 0;

    if t > sum_a {
        let div_a: isize = t / sum_a;
        t -= div_a * sum_a;
    }

    for i in 0..n {
        if t > a[i] {
            t -= a[i];
            mus += 1;
        } else if t == a[i] {
            mus_t = a[i];
            break;
        } else {
            mus_t = t;
            break;
        }
    }

    println!("{} {}", mus, mus_t);
}

#[allow(dead_code)]
fn abc186_c() {
    input! {
        n: usize,
    }

    let mut ans: usize = 0;

    for i in 1..=n {
        let mut flag: bool = true;

        for base in [8, 10] {
            let mut x = i;
            while x > 0 {
                if x % base == 7 {
                    flag = false;
                }
                x /= base;
            }
        }
        if flag {
            ans += 1;
        }
    }
    println!("{}", ans);
}

#[allow(dead_code)]
fn arc143() {
    input! {
        mut abc: [usize; 3],
    }

    abc.sort();

    if abc[0] + abc[1] >= abc[2] {
        println!("{}", abc[2]);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
fn abc099() {
    input! {
        a: usize,
        b: usize,
    }

    let d = b - a;
    let ans = d * (d + 1) / 2 - b;

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc012() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    let tmp: usize = a;
    a = b;
    b = tmp;

    println!("{} {}", a, b);
}

#[allow(dead_code)]
fn arc009() {
    input! {
        n: usize,
    }

    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();

    for _ in 0..n {
        input! {
            a_i: usize,
            b_i: usize,
        }

        a.push(a_i);
        b.push(b_i);
    }

    let mut ans: usize = 0;
    for i in 0..n {
        ans += a[i] * b[i];
    }

    println!("{}", (ans as f64 * 1.05) as usize);
}

#[allow(dead_code)]
fn algo368() {
    let n: f64 = read_line()[0];

    let mut left: f64 = 0.0;
    let mut right: f64 = 100.0;
    while right - left > 1e-4 {
        let mid: f64 = (left + right) / 2.0;

        let f: f64 = mid * (mid * (mid + 1.0) + 2.0) + 3.0;

        if f < n {
            left = mid;
        } else {
            right = mid;
        }
    }

    let ans: f64 = left;

    println!("{}", ans);
}

#[allow(dead_code)]
fn algo363() {
    let n: usize = read_line()[0];

    let mut sa: Vec<i32> = Vec::new();
    let mut ta: Vec<i32> = Vec::new();

    for _ in 0..n {
        let line: Vec<i32> = read_line();
        sa.push(line[0]);
        ta.push(line[1]);
    }

    let mut ids: Vec<usize> = (0..n).collect();

    ids.sort_by(|&i, &j| ta[i].cmp(&ta[j]));

    println!("{:?}", ids);
    println!("{:?}", ta);
    println!("{:?}", sa);

    let mut res: usize = 0;
    let mut lt: i32 = 0;
    for &i in &ids {
        if sa[i] < lt {
            continue;
        }

        res += 1;
        lt = ta[i];
    }

    println!("{}", res);
}

#[allow(dead_code)]
fn algo365() {
    let n: usize = read_line()[0];

    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    for _ in 0..n {
        let line: Vec<f64> = read_line();
        x.push(line[0]);
        y.push(line[1]);
    }

    let mut asn: f64 = 0.0;

    let mut used: Vec<bool> = vec![false; n];
    used[0] = true;

    let mut prev: usize = 0;

    for _ in 0..n - 1 {
        let mut nex: i32 = -1;
        let mut min_dis: f64 = 100000000.0;

        for i in 0..n {
            if used[i] {
                continue;
            }

            let dis: f64 =
                ((x[i] - x[prev]) * (x[i] - x[prev]) + (y[i] - y[prev]) * (y[i] - y[prev])).sqrt();

            if dis < min_dis {
                min_dis = dis;
                nex = i as i32;
            }
        }

        used[nex as usize] = true;
        asn += min_dis;

        prev = nex as usize;
    }

    asn += ((x[0] - x[prev]) * (x[0] - x[prev]) + (y[0] - y[prev]) * (y[0] - y[prev])).sqrt();

    println!("{}", asn);
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
