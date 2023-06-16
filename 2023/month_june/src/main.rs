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

fn main() {
    agc019_a();
}

#[allow(dead_code)]
fn agc019_a() {
    input! {
        q: i64,
        h: i64,
        s: i64,
        d: i64,
        n: i64,
    }

    let a = cmp::min(q * 4, cmp::min(h * 2, s));

    println!("{}", cmp::min(n * a, (n / 2 * d) + (n % 2 * a)));
}

#[allow(dead_code)]
fn abc100_c() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut cnt = 0;

    for e in a.iter() {
        let mut tmp_e = *e;
        loop {
            if tmp_e % 2 != 0 {
                break;
            }

            tmp_e /= 2;
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn abc144_c() {
    input! {
        n: usize,
    }

    let mut ans: usize = 1e18 as usize;

    for i in 1..=n {
        if i * i > n {
            break;
        }
        if n % i != 0 {
            continue;
        }

        let j = n / i;
        ans = cmp::min(ans, i + j - 2);
    }
    println!("{}", ans);
}

#[allow(dead_code)]
fn agc041_a() {
    input! {
        n: u128,
        a: u128,
        b: u128,
    }

    if a % 2 == b % 2 {
        println!("{}", (b - a) / 2);
    } else {
        println!("{}", cmp::min(a - 1, n - b) + 1 + (b - a - 1) / 2);
    }
}

#[allow(dead_code)]
fn abc136_c() {
    input! {
        n: usize,
        mut h: [i32; n],
    }

    let mut pre = -999;
    for i in 0..n {
        if pre <= h[i] - 1 {
            pre = h[i] - 1;
        } else if pre <= h[i] {
            pre = h[i];
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

#[allow(dead_code)]
fn agc012_a() {
    input! {
        n: usize,
        a: [i64; 3*n],
    }

    let mut a = a;
    a.sort();

    let mut sum = 0;
    let mut count = 0;
    for i in n..3 * n {
        if count % 2 == 0 {
            sum += a[i];
        }
        count += 1;
    }

    println!("{}", sum);
}

#[allow(dead_code)]
fn abc151_c() {
    input! {
        n: usize,
        m: i64,
        p: [(usize, String); m],
    }

    let mut correct_arr: Vec<bool> = vec![false; n];
    let mut pena_arr: Vec<isize> = vec![0; n];

    if m != 0 {
        for (i, j) in p.iter() {
            if !correct_arr[i - 1] {
                if j == "AC" {
                    correct_arr[i - 1] = true;
                } else {
                    pena_arr[i - 1] += 1;
                }
            } else {
                continue;
            }
        }
    }

    let mut pena = 0;
    for (flg, i) in correct_arr.iter().zip(0..n) {
        if *flg {
            pena += pena_arr[i];
        } else {
            continue;
        }
    }

    let correct_num = correct_arr.iter().filter(|&x| *x).count();
    println!("{} {}", correct_num, pena);
}

#[allow(dead_code)]
fn abc115_c() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    h.sort();

    let mut ans: usize = 10000000009;

    for i in 0..=h.len() - k {
        let tmp = h[i + (k - 1)] - h[i];
        ans = cmp::min(ans, tmp);
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc095_c() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
        y: i32,
    }

    let ans: i32;

    if a + b > c * 2 {
        if x > y {
            let tmp1 = c * (x * 2);
            let dis = x - y;
            let tmp2 = (c * (y * 2)) + (a * dis);
            ans = cmp::min(tmp1, tmp2);
        } else {
            let tmp1 = c * (y * 2);
            let dis = y - x;
            let tmp2 = (c * (x * 2)) + (b * dis);
            ans = cmp::min(tmp1, tmp2);
        }
    } else {
        ans = a * x + b * y;
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
            let first_digit = b.chars().next().unwrap().to_digit(10).unwrap() as i64;
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