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

const MOD: usize = 1_000_000_000;

fn main() {
    abc130b();
}

#[allow(dead_code)]
fn abc095b() {}

#[allow(dead_code)]
fn abc130b() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n]
    }

    let mut d = 0;
    let mut ans = 0;

    for i in 0..n {
        if d <= x {
            ans += 1;
        }

        d += a[i];
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc174b() {
    input! {
        n: usize,
        d: usize,
        xy: [(isize, isize); n]
    }

    let mut ans = 0;

    for i in 0..n {
        let sq = (xy[i].0 * xy[i].0 + xy[i].1 * xy[i].1) as f64;
        let sq_root = sq.sqrt();
        if sq_root <= d as f64 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc088b() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut alice = 0;
    let mut bob = 0;

    a.sort_by(|a, b| b.cmp(a));

    for i in 0..n {
        if i % 2 == 0 {
            alice += a[i];
        } else {
            bob += a[i];
        }
    }

    println!("{}", alice - bob);
}

#[allow(dead_code)]
fn abc081b() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut dev_cnt = 0;

    loop {
        let mut flg = true;
        for i in 0..n {
            if a[i] % 2 != 0 {
                flg = false;
                break;
            }
        }
        if flg {
            for i in 0..n {
                a[i] /= 2;
            }
            dev_cnt += 1;
        } else {
            break;
        }
    }

    println!("{}", dev_cnt);
}

#[allow(dead_code)]
fn abc220b() {
    input! {
        k: usize,
        mut a: usize,
        mut b: usize
    }

    let mut i = 0;
    let mut a_decimal = 0;
    let mut b_decimal = 0;
    while a > 0 {
        let a_tmp = a % 10;
        a_decimal += a_tmp * k.pow(i as u32);
        a /= 10;
        i += 1;
    }

    i = 0;
    while b > 0 {
        let b_tmp = b % 10;
        b_decimal += b_tmp * k.pow(i as u32);
        b /= 10;
        i += 1;
    }

    let ans = a_decimal * b_decimal;
    println!("{}", ans);
}

#[allow(dead_code)]
fn abc090b() {
    input! {
        a: usize,
        b: usize
    }

    let mut palindrome_number = 0;

    for i in a..=b {
        let s = i.to_string();
        let s_rev = s.chars().rev().collect::<String>();
        if s == s_rev {
            palindrome_number += 1;
        }
    }

    println!("{}", palindrome_number);
}

#[allow(dead_code)]
fn abc068b() {
    input! {
        n: usize
    }

    let mut div_max_cnt = 0;
    let mut div_max = 1;

    for i in 1..=n {
        let mut div_i = i;
        let mut div_cnt = 0;
        while div_i / 2 != 0 {
            div_i /= 2;
            div_cnt += 1;
        }

        if div_cnt > div_max_cnt {
            div_max_cnt = div_cnt;
            div_max = i;
        }
    }

    println!("{}", div_max);
}

#[allow(dead_code)]
fn abc158c() {
    input! {
        a: usize,
        b: usize
    }

    let mut money: f64 = 1_f64;
    loop {
        let a_tax = money * 0.08;
        let b_tax = money * 0.1;

        let a_tax = a_tax.floor() as usize;
        let b_tax = b_tax.floor() as usize;

        if a_tax == a && b_tax == b {
            println!("{}", money);
            return;
        }

        if a_tax > a || b_tax > b {
            println!("-1");
            return;
        }

        money += 1_f64;
    }
}

#[allow(dead_code)]
fn abc093b() {
    input! {
        a: usize,
        b: usize,
        k: usize
    }

    if (b - a + 1) <= k {
        for i in a..=b {
            println!("{}", i);
        }
        return;
    }

    let mut sub_vec = vec![];
    let mut cnt = 0;
    for i in a..=b {
        if cnt < k {
            sub_vec.push(i);
            cnt += 1;
        } else {
            break;
        }
    }

    cnt = 0;
    for i in (a..=b).rev() {
        if cnt < k {
            sub_vec.push(i);
            cnt += 1;
        } else {
            break;
        }
    }

    sub_vec.sort();
    sub_vec.dedup();

    for i in sub_vec {
        println!("{}", i);
    }
}

#[allow(dead_code)]
fn abc208b() {
    input! {
        mut p: usize
    }

    let factorial_10 = (1..=10).fold(1, |acc, x| acc * x);
    let factorial_9 = (1..=9).fold(1, |acc, x| acc * x);
    let factorial_8 = (1..=8).fold(1, |acc, x| acc * x);
    let factorial_7 = (1..=7).fold(1, |acc, x| acc * x);
    let factorial_6 = (1..=6).fold(1, |acc, x| acc * x);
    let factorial_5 = (1..=5).fold(1, |acc, x| acc * x);
    let factorial_4 = (1..=4).fold(1, |acc, x| acc * x);
    let factorial_3 = (1..=3).fold(1, |acc, x| acc * x);
    let factorial_2 = (1..=2).fold(1, |acc, x| acc * x);
    let factorial_1 = (1..=1).fold(1, |acc, x| acc * x);

    let mut cnt = 0;

    loop {
        if p >= factorial_10 {
            p -= factorial_10;
            cnt += 1;
        } else if p >= factorial_9 {
            p -= factorial_9;
            cnt += 1;
        } else if p >= factorial_8 {
            p -= factorial_8;
            cnt += 1;
        } else if p >= factorial_7 {
            p -= factorial_7;
            cnt += 1;
        } else if p >= factorial_6 {
            p -= factorial_6;
            cnt += 1;
        } else if p >= factorial_5 {
            p -= factorial_5;
            cnt += 1;
        } else if p >= factorial_4 {
            p -= factorial_4;
            cnt += 1;
        } else if p >= factorial_3 {
            p -= factorial_3;
            cnt += 1;
        } else if p >= factorial_2 {
            p -= factorial_2;
            cnt += 1;
        } else if p >= factorial_1 {
            p -= factorial_1;
            cnt += 1;
        } else {
            break;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn abc164b() {
    input! {
        mut a: usize,
        b: usize,
        mut c: usize,
        d: usize
    }

    loop {
        // hero turn
        c = c.saturating_sub(b);
        if c == 0 {
            println!("Yes");
            return;
        }
        // monstaer turn
        a = a.saturating_sub(d);
        if a == 0 {
            println!("No");
            return;
        }
    }
}

#[allow(dead_code)]
fn abc200b() {
    input! {
        mut n: usize,
        k: usize
    }

    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = n * 1000 + 200;
        }
    }

    println!("{}", n);
}

#[allow(dead_code)]
fn abc165b() {
    input! {
        x: usize,
    }

    let mut ans = 100;
    let mut i = 1;

    while ans < x {
        ans += ans / 100;
        i += 1;
    }

    println!("{}", i - 1);
}

#[allow(dead_code)]
fn abc206b() {
    input! {
        n: usize
    }

    let mut sum = 1;

    let mut i = 1;
    while n >= sum {
        sum += i;
        i += 1;
    }

    println!("{}", i - 1);
}

#[allow(dead_code)]
fn abc162b() {
    input! {
        n: usize,
    }

    let mut sum = 0;

    for i in 1..=n {
        if i % 3 == 0 || i % 5 == 0 {
            continue;
        }
        sum += i;
    }

    println!("{}", sum);
}

#[allow(dead_code)]
fn abc153a() {
    input! {
        h: usize,
        a: usize,
    }

    if h < a {
        println!("1");
        return;
    }

    let mut ans = h / a;

    if h % a != 0 {
        ans += 1;
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn dfs(idx: usize, k: usize, n: usize, s: &[u8], pos: &mut Vec<usize>, all: &mut Vec<Vec<usize>>) {
    if pos.len() == k {
        all.push(pos.clone());
        return;
    }
    if idx >= n {
        return;
    }

    dfs(idx + 1, k, n, s, pos, all);

    if s[idx] == b'.' {
        return;
    }
    if !pos.is_empty() && idx <= pos[pos.len() - 1] + 1 {
        return;
    }

    pos.push(idx);
    dfs(idx + 1, k, n, s, pos, all);
    pos.pop();
}

#[allow(dead_code)]
// fn abc401d() {
//     const DOT: u8 = b'.';
//     const O: u8 = b'o';
//     const Q: u8 = b'?';

//     input! {
//         n: usize,
//         k: usize,
//         s: [u8; n],
//     }

//     let mut count_o = vec![0; n];
//     let mut count_dot = vec![0; n];
//     let mut total = 0;

//     let mut positions = vec![];

//     let mut all_valid_o_positions = vec![];
//     dfs(0, k, n, s, &mut positions, &mut all_valid_o_positions);

//     'outer: for o_pos in all_valid_o_positions {
//         for (i, &c) in s.iter().enumerate() {
//             if c == O && !o_pos.contains(&i) {
//                 continue 'outer;
//             }
//             if c == DOT && o_pos.contains(&i) {
//                 continue 'outer;
//             }
//         }
//         total += 1;
//         let mut temp = vec![DOT; n];
//         for &i in &o_pos {
//             temp[i] = O;
//         }
//         for i in 0..n {
//             match temp[i] {
//                 O => count_o[i] += 1,
//                 DOT => count_dot[i] += 1,
//                 _ => {}
//             }
//         }
//     }

//     let mut res = vec![b'?'; n];
//     for i in 0..n {
//         if count_o[i] = total {
//             res[i] = b'o';
//         } else if count_dot[i] = total {
//             res[i] = b'.';
//         }
//     }

//     println!("{}", String::from_utf8(res).unwrap());
// }
#[allow(dead_code)]
fn abc401c() {
    input! {
        n: usize,
        k: usize
    }

    let mut a = vec![0usize; n + 1];
    let mut s = vec![0usize; n + 2];

    for i in 0..=n {
        if i < k {
            a[i] = 1;
        } else {
            a[i] = (s[i] + MOD - s[i - k]) % MOD;
        }
        s[i + 1] = (s[i] + a[i]) % MOD;
    }

    println!("{}", a[n]);
}

#[allow(dead_code)]
fn abc401b() {
    input! {
        n: usize,
        s_arr: [String; n],
    }

    let mut login_flg = false;

    let mut flg_false_private_access_cnt = 0;

    for s in s_arr {
        if s == "login" {
            login_flg = true;
            continue;
        }

        if s == "logout" {
            login_flg = false;
            continue;
        }

        if s == "private" {
            if !login_flg {
                flg_false_private_access_cnt += 1;
            }
            continue;
        }
    }

    println!("{}", flg_false_private_access_cnt);
}

#[allow(dead_code)]
fn abc401a() {
    input! {
        s: usize,
    }

    if s >= 200 && s <= 299 {
        println!("Success");
        return;
    }

    println!("Failure");
}

#[allow(dead_code)]
fn abc086a() {
    input! {
        a: usize,
        b: usize,
    }

    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

#[allow(dead_code)]
fn abc169a() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", a * b);
}
