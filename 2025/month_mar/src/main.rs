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

fn main() {
    abc304a();
}

#[allow(dead_code)]
fn abc304a() {
    input! {
        n: usize,
        pairs: [(String, usize); n]
    }

    let min_age_index = pairs
        .iter()
        .enumerate()
        .min_by_key(|&(_, (_, age))| age)
        .map(|(index, _)| index)
        .unwrap();

    let mut people: Vec<String> = Vec::new();

    for i in min_age_index..n {
        people.push(pairs[i].0.clone());
    }

    for i in 0..min_age_index{
        people.push(pairs[i].0.clone());
    }

    for elem in people.iter() {
        println!("{}", elem);
    }
}

#[allow(dead_code)]
fn abc305a() {
    input! {
        n: usize,
    }

    if n == 100 {
        println!("100");
        return;
    }

    if n >= 10 {
        let n_10 = n / 10;
        let n_1 = n % 10;

        if n_1 == 0 || n_1 == 1 || n_1 == 2 {
            let ans = n_10 * 10;
            println!("{}", ans);
        } else if n_1 == 3 || n_1 == 4 || n_1 == 5 {
            let ans = n_10 * 10 + 5;
            println!("{}", ans);
        } else if n_1 == 6 || n_1 == 7 {
            let ans = n_10 * 10 + 5;
            println!("{}", ans);
        } else if n_1 == 8 || n_1 == 9 {
            let ans = n_10 * 10 + 10;
            println!("{}", ans);
        } 
    } else {
        if n == 0 || n == 1 || n == 2 {
            println!("0");
        } else if n == 3 || n == 4 || n == 5 {
            println!("5");
        } else if n == 6 || n == 7 {
            println!("5");
        } else if n == 8 || n == 9 {
            println!("10");
        }
    }
}

#[allow(dead_code)]
fn isqrt(n: usize) -> usize {
    let mut low = 0;
    let mut high = n;
    while low <= high {
        let mid = (low + high) / 2;

        if mid == 0 {
            low = 1;
            continue;
        }

        if mid <= n / mid {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    high
}

#[allow(dead_code)]
fn abc400c() {
    input! {
        n: usize
    }

    let mut gn = HashSet::new();

    let mut pow2 = 1;
    while pow2 <= n {
        let max_b = (n / pow2) as f64;
        let b_limit = max_b.sqrt() as usize;

        for b in 1..=b_limit {
            let x = pow2 * b * b;
            if x <= n {
                gn.insert(x);
            }
        }

        match pow2.checked_mul(2) {
            Some(p) => pow2 = p,
            None => break,
        }
    }

    println!("{}", gn.len());
}

#[allow(dead_code)]
fn abc400b() {
    input! {
        n: usize,
        m: usize,
    }

    let inf_judge = 10usize.pow(9);

    let mut sum = 0;

    for i in 0..=m {
        sum += n.pow(i as u32);
        if sum > inf_judge {
            println!("inf");
            return;
        }
    }

    println!("{}", sum);
}

#[allow(dead_code)]
fn abc400a() {
    input! {
        a: usize,
    }

    if 400 % a != 0 {
        println!("-1");
        return;
    }

    let ans = 400 / a;

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc306a() {
    input! {
        _: usize,
        s: String
    }

    let mut ans_str = String::new();

    for c in s.chars() {
        ans_str.push(c);
        ans_str.push(c);
    }

    println!("{}", ans_str);
}

#[allow(dead_code)]
fn abc307a() {
    input! {
        n: usize,
        a: [usize; n*7]
    }

    let mut sum_vec: Vec<usize> = Vec::new();

    let mut sum = 0;
    for (i, elem) in a.iter().enumerate() {
        sum += elem;
        if (i + 1) % 7 == 0 {
            sum_vec.push(sum);
            sum = 0;
        }
    }

    let mut ans_str = String::new();

    for elem in sum_vec.iter() {
        ans_str.push_str(&elem.to_string());
        ans_str.push(' ');
    }

    println!("{}", ans_str.trim_end());
}

#[allow(dead_code)]
fn abc308a() {
    input! {
       s: [usize; 8],
    }

    for i in 0..7 {
        if s[i] > s[i + 1] {
            println!("No");
            return;
        }
    }

    if s.iter().any(|&x| x < 100 || x > 675) {
        println!("No");
        return;
    }

    if s.iter().any(|&x| x % 25 != 0) {
        println!("No");
        return;
    }

    println!("Yes");
}

#[allow(dead_code)]
fn abc309a() {
    input! {
        a: usize,
        b: usize,
    }

    let pair = (a, b);

    match pair {
        (1, 2) => println!("Yes"),
        (2, 3) => println!("Yes"),
        (4, 5) => println!("Yes"),
        (5, 6) => println!("Yes"),
        (7, 8) => println!("Yes"),
        (8, 9) => println!("Yes"),
        _ => println!("No"),
    }
}

#[allow(dead_code)]
fn abc310a() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        mut d: [usize; n]
    }

    for elem in d.iter_mut() {
        *elem += q;
    }

    let min_d = d.iter().min().unwrap();

    if p >= *min_d {
        println!("{}", *min_d);
    } else {
        println!("{}", p);
    }
}

#[allow(dead_code)]
fn abc311a() {
    input! {
        _: usize,
        s: String
    }

    let mut a_cnt = 0;
    let mut b_cnt = 0;
    let mut c_cnt = 0;

    let mut idx = 0;

    for c in s.chars() {
        idx += 1;
        if c == 'A' && a_cnt == 0 {
            a_cnt = 1;
        }
        if c == 'B' && b_cnt == 0 {
            b_cnt = 1;
        }
        if c == 'C' && c_cnt == 0 {
            c_cnt = 1;
        }

        if a_cnt == 1 && b_cnt == 1 && c_cnt == 1 {
            break;
        }
    }

    println!("{}", idx);
}

#[allow(dead_code)]
fn abc314a() {
    input! {
        n: usize
    }

    let pi100: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";

    let s1 = &pi100[0..n + 2];

    println!("{}", s1);
}

#[allow(dead_code)]
fn abc317a() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        mut p: [usize; n]
    }

    let sub_a = x - h;
    let mut ans: usize = 0;

    for (pa, num) in zip(p.iter_mut(), 1..) {
        if *pa >= sub_a {
            ans = num;
            break;
        }
    }

    println!("{}", ans);
}
