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
    abc322_b_ans();
}

#[allow(dead_code)]
fn abc322_b_ans() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }

    let mut is_prefix = true;
    for i in 0..n {
        if s.chars().nth(i) != t.chars().nth(i) {
            is_prefix = false;
        }
    }
    let mut is_suffix = true;
    for i in 0..n {
        if s.chars().nth(i) != t.chars().nth(m - n + i) {
            is_suffix = false;
        }
    }

    if is_prefix {
        println!("{}", if is_suffix { 0 } else { 1 });
    } else {
        println!("{}", if is_suffix { 2 } else { 3 });
    }
}

#[allow(dead_code)]
fn abc322_b() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }

    let sc: Vec<char> = s.chars().collect();
    let tc: Vec<char> = t.chars().collect();

    let reverse_sc: Vec<char> = sc.iter().rev().map(|c| *c).collect();
    let reverse_tc: Vec<char> = tc.iter().rev().map(|c| *c).collect();

    if s == t {
        println!("0");
        return;
    }

    for i in 0..=m - n {
        let mut tmp_a = 0;
        let mut tmp_b = 0;
        for j in 0..n {
            if sc[j] == tc[i + j] {
                tmp_a += 1;
            }
        }

        if tmp_a == n {
            println!("1");
            return;
        }

        for j in 0..n {
            if reverse_sc[j] == reverse_tc[i + j] {
                tmp_b += 1;
            }
        }

        if tmp_b == n {
            println!("2");
            return;
        }
    }

    println!("3");
}

#[allow(dead_code)]
fn check_sudoku(a: &Vec<Vec<usize>>) -> bool {
    for i in 0..9 {
        let mut row = vec![false; 9];
        let mut col = vec![false; 9];
        let mut block = vec![false; 9];

        for j in 0..9 {
            let row_idx = a[i][j] - 1;
            if row[row_idx] {
                return false;
            }
            row[row_idx] = true;

            let col_idx = a[j][i] - 1;
            if col[col_idx] {
                return false;
            }
            col[col_idx] = true;

            let block_row_idx = (i / 3) * 3 + j / 3;
            let block_col_idx = (i % 3) * 3 + j % 3;
            let block_idx = a[block_row_idx][block_col_idx] - 1;
            if block[block_idx] {
                return false;
            }
            block[block_idx] = true;
        }
    }
    true
}

#[allow(dead_code)]
fn abc327_c() {
    input! {
        a: [[usize; 9]; 9],
    }

    let ans = check_sudoku(&a);

    println!("{}", if ans { "Yes" } else { "No" });
}

#[allow(dead_code)]
fn abc327_b() {
    input! {
        n: usize,
    }

    let mut ans: isize = -1;

    for i in 1..=n {
        let tmp: usize = i.pow(i as u32);
        if n == tmp {
            ans = i as isize;
            break;
        }

        if tmp > n {
            break;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc327_a() {
    input! {
        n: usize,
        s: String,
    }

    let s: Vec<char> = s.chars().collect();

    let mut ans: String = "No".to_string();

    for i in 0..n - 1 {
        if (s[i] == 'a' && s[i + 1] == 'b') || (s[i] == 'b' && s[i + 1] == 'a') {
            ans = "Yes".to_string();
            break;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc322_a() {
    input! {
        n: usize,
        s: String,
    }

    let s: Vec<char> = s.chars().collect();

    let mut ans: i32 = -1;

    for i in 0..n - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            ans = i as i32 + 1;
            break;
        }
    }

    println!("{}", ans);
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
