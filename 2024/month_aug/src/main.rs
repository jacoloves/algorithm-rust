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
    abc366_b();
}

#[allow(dead_code)]
fn abc366_b() {
    input! {
        n: usize,
        s: [String; n],
    }

    let m = s.iter().map(|e| e.len()).max().unwrap();
    let mut t = vec![vec!['*'; n]; m];

    for i in 0..n {
        let chars: Vec<char> = s[i].chars().collect();
        for j in 0..chars.len() {
            t[j][n - i - 1] = chars[j];
        }
    }

    for i in 0..m {
        while t[i].last() == Some(&'*') {
            t[i].pop();
        }
        println!("{}", t[i].iter().collect::<String>());
    }
}

#[allow(dead_code)]
fn abc366_a() {
    input! {
        n: usize,
        t: usize,
        a: usize
    }

    if t == a {
        println!("No");
        return;
    }

    let sum_cnt = t + a;
    let rest = n - sum_cnt;

    if t < a {
        if a - t <= rest {
            println!("No");
        } else {
            println!("Yes");
        }
    } else if t - a <= rest {
        println!("No");
    } else {
        println!("Yes");
    }
}

#[allow(dead_code)]
fn abc365_b() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut copy_a = a.clone();

    copy_a.sort();

    let mut second_max = 0;
    for i in 0..n {
        if a[i] == copy_a[n - 2] {
            second_max = i;
            break;
        }
    }

    println!("{}", second_max + 1);
}

#[allow(dead_code)]
fn abc365_a() {
    input! {
        y: usize,
    }

    if y % 4 == 0 && y % 100 != 0 || y % 400 == 0 {
        println!("366");
    } else {
        println!("365");
    }
}

#[allow(dead_code)]
fn abc364_b() {
    input! {
        h: isize,
        w: isize,
        si: isize,
        sj: isize,
        sc: [String; h],
        x: String,
    }

    let c = sc
        .iter()
        .map(|e| e.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut takah_point = [si - 1, sj - 1];

    for xc in x.chars() {
        match xc {
            'U' => {
                if takah_point[0] - 1 < 0 {
                } else if c[takah_point[0] as usize - 1][takah_point[1] as usize] == '#' {
                    continue;
                } else {
                    takah_point[0] -= 1;
                }
            }
            'D' => {
                if takah_point[0] + 1 >= h {
                } else if c[takah_point[0] as usize + 1][takah_point[1] as usize] == '#' {
                    continue;
                } else {
                    takah_point[0] += 1;
                }
            }
            'L' => {
                if (takah_point[1] - 1) < 0 {
                } else if c[takah_point[0] as usize][takah_point[1] as usize - 1] == '#' {
                    continue;
                } else {
                    takah_point[1] -= 1;
                }
            }
            'R' => {
                if takah_point[1] + 1 >= w {
                } else if c[takah_point[0] as usize][takah_point[1] as usize + 1] == '#' {
                    continue;
                } else {
                    takah_point[1] += 1;
                }
            }
            _ => {}
        }
    }

    println!("{} {}", takah_point[0] + 1, takah_point[1] + 1);
}

#[allow(dead_code)]
fn abc364_a() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut cnt = 0;
    let mut over_cnt = 0;
    for e in s.iter() {
        if e == "sweet" {
            over_cnt += 1;
            cnt += 1;
        } else {
            over_cnt = 0;
            cnt += 1
        }

        if over_cnt == 2 && cnt < n {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

#[allow(dead_code)]
fn abc363_b() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut l: [usize; n],
    }

    let mut ans = 0;

    loop {
        let mut sum_l = 0;
        for e in l.iter_mut() {
            if *e >= t {
                sum_l += 1;
            }
        }

        if sum_l >= p {
            break;
        } else {
            for e in l.iter_mut() {
                *e += 1;
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc363_a() {
    input! {
        r: usize,
    }

    if r < 100 {
        println!("{}", 100 - r);
    } else if r < 200 {
        println!("{}", 200 - r);
    } else {
        println!("{}", 300 - r);
    }
}

#[allow(dead_code)]
fn abc362_b() {
    input! {
        xa: isize,
        ya: isize,
    }

    input! {
        xb: isize,
        yb: isize,
    }

    input! {
        xc: isize,
        yc: isize,
    }

    let ab2 = (xa - xb).pow(2) + (ya - yb).pow(2);
    let bc2 = (xb - xc).pow(2) + (yb - yc).pow(2);
    let ca2 = (xc - xa).pow(2) + (yc - ya).pow(2);

    if ab2 + bc2 == ca2 || bc2 + ca2 == ab2 || ca2 + ab2 == bc2 {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn abc362_a() {
    input! {
        r: usize,
        g: usize,
        b: usize
    }

    input! {
        c: String,
    }

    let ans: usize;
    if c == "Red" {
        ans = if g < b { g } else { b };
    } else if c == "Green" {
        ans = if r < b { r } else { b };
    } else {
        ans = if r < g { r } else { g };
    }

    println!("{}", ans);
}
