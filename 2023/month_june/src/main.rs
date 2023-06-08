#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(unused_imports)]
use std::cmp;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]

fn main() {
    abc095_a();
}

#[allow(dead_code)]
fn abc095_a() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
        y: i32,
    }

    let ans: i32;

    if a+b > c*2 {
        if x > y {
            let tmp1 = c * (x*2);
            let dis = x-y;
            let tmp2 = (c * (y*2)) + (a * dis);
            ans = cmp::min(tmp1, tmp2);
        } else {
            let tmp1 = c * (y*2);
            let dis = y-x;
            let tmp2 = (c * (x*2)) + (b * dis);
            ans = cmp::min(tmp1, tmp2);
        }
    } else {
        ans = a*x + b*y;
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn agc021_a() {
    input! {
        n: i64,
    }

    if n < 10 {
        println!("{}", n);
    } else {
        let a = n;
        let mut digits = 0;
        let mut temp = a;
        while temp > 0 {
            digits += 1;
            temp /= 10;
        }

        let b = n.to_string();
        let mut flag = false;
        for (_, c) in b.chars().enumerate().skip(1) {
            if c != '9' {
                flag = true;
                break;
            }
        }

        if !flag && b.chars().next().unwrap() == '9' {
            println!("{}", digits * 9);
        } else if !flag && b.chars().next().unwrap() != '9' {
            let first_digit = b.chars().next().unwrap().to_digit(10).unwrap() as i64;
            println!("{}", (digits - 1) * 9 + first_digit);
        } else {
            let first_digit  = b.chars().next().unwrap().to_digit(10).unwrap() as i64;
            println!("{}", (digits - 1) * 9 + first_digit - 1);
        }
    }
}

#[allow(dead_code)]
fn abc097_b() {
    input! {
        x: usize,
    }

    let mut ans = 0;

    for i in 1..=x {
        for j in 2..=1000 {
            let t = i.pow(j as u32);
            if x >= t {
                ans = cmp::max(ans, t);
            } else {
                break;
            }
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc042_b() {
    input! {
        n: usize,
        _l: usize,
        mut s: [String; n],
    }

    s.sort();

    let mut ans: String = "".to_string();

    for e in s.iter() {
        ans += e;
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn sumitb2019_c() {
    input! {
        x: usize,
    }

    let mut dp = vec![false; 101010];
    dp[0] = true;

    for i in 0..x {
        if dp[i] {
            for d in 0..6 {
                dp[i + d + 100] = true;
            }
        }
    }

    if dp[x] {
        println!("1");
    } else {
        println!("0");
    }
}
