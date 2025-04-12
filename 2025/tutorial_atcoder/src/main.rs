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
    abc401c();
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
fn abc401d() {
    const DOT: u8 = b'.';
    const O: u8 = b'o';
    const Q: u8 = b'?';

    input! {
        n: usize,
        k: usize,
        s: [u8; n],
    }

    let mut count_o = vec![0; n];
    let mut count_dot = vec![0; n];
    let mut total = 0;

    let mut positions = vec![];

    let mut all_valid_o_positions = vec![];
    dfs(0, k, n, s, &mut positions, &mut all_valid_o_positions);

    'outer: for o_pos in all_valid_o_positions {
        for (i, &c) in s.iter().enumerate() {
            if c == O && !o_pos.contains(&i) {
                continue 'outer;
            }
            if c == DOT && o_pos.contains(&i) {
                continue 'outer;
            }
        }
        total += 1;
        let mut temp = vec![DOT; n];
        for &i in &o_pos {
            temp[i] = O;
        }
        for i in 0..n {
            match temp[i] {
                O => count_o[i] += 1,
                DOT => count_dot[i] += 1,
                _ => {}
            }
        }
    }

    let mut res = vec![b'?'; n];
    for i in 0..n {
        if count_o[i] = total {
            res[i] = b'o';
        } else if count_dot[i] = total {
            res[i] = b'.';
        }
    }

    println!("{}", String::from_utf8(res).unwrap());
}

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
