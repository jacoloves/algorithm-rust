mod past;
use crate::past::week1::abc158_b;

use itertools::Itertools;
use std::cmp;
use std::cmp::min;
use std::fmt::Debug;
use std::io::empty;
use std::ptr::read;
use std::str::FromStr;

fn read_line<T: std::str::FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}

fn main() {
    abc071_b();
}

fn abc071_b() {
    let l: Vec<String> = read_line();
    let s = l[0].clone();

    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut flg = false;

    for c in alphabet.chars() {
        let ans_match = s.find(c);
        if ans_match == None {
            println!("{}", c);
            flg = true;
            break;
        }
    }

    if !flg {
        println!("None");
    }
}

fn abc087_b() {
    let l: Vec<usize> = read_line();
    let a = l[0];
    let l: Vec<usize> = read_line();
    let b = l[0];
    let l: Vec<usize> = read_line();
    let c = l[0];
    let l: Vec<usize> = read_line();
    let x = l[0];

    let mut cnt = 0;
    for a_i in 0..=a {
        for b_i in 0..=b {
            for c_i in 0..=c {
                if x == 500 * a_i + 100 * b_i + 50 * c_i {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}

fn abc084_b() {
    let l: Vec<usize> = read_line();
    let a = l[0];
    let b = l[1];

    let l: Vec<String> = read_line();
    let s = l[0].clone();

    let n = a + b + 1;
    let mut flg = true;
    for (c, i) in s.chars().zip(1..=n) {
        if i == a + 1 {
            if c != '-' {
                flg = false;
                break;
            }
        } else if c == '-' {
            flg = false;
            break;
        }
    }

    if flg {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn abc052_b() {
    let _l: Vec<usize> = read_line();
    let l: Vec<String> = read_line();
    let s = l[0].clone();

    let mut ans = 0;

    let mut cnt = 0;
    for c in s.chars() {
        if c == 'D' {
            cnt -= 1;
        } else if c == 'I' {
            cnt += 1;
        }

        ans = cmp::max(ans, cnt);
    }

    println!("{}", ans);
}

fn abc063_b() {
    let l: Vec<String> = read_line();
    let s = l[0].clone();

    let s_copy = s.clone();

    let mut flg = true;
    for se in s.chars() {
        let mut cnt = 0;
        for sce in s_copy.chars() {
            if se == sce {
                cnt += 1;
            }
        }

        if cnt >= 2 {
            flg = false;
            break;
        }
    }

    if flg {
        println!("yes");
    } else {
        println!("no");
    }
}

fn abc108_b() {
    let l: Vec<isize> = read_line();
    let x_1 = l[0];
    let y_1 = l[1];
    let x_2 = l[2];
    let y_2 = l[3];

    let mut dx = x_2 - x_1;
    let mut dy = y_2 - y_1;
    let mut x = x_2;
    let mut y = y_2;

    for i in 0..2 {
        let dx_2 = -dy;
        let dy_2 = dx;
        dx = dx_2;
        dy = dy_2;

        x = x + dx;
        y = y + dy;

        print!("{} {}", x, y);
        if i == 0 {
            print!(" ");
        } else {
            println!();
        }
    }
}

fn abc114_b_f(a: char, b: char, c: char) -> f32 {
    let marge_s = format!("{}{}{}", a, b, c);

    let marge_num: f32 = marge_s.parse().unwrap();

    return (marge_num - 753 as f32).abs();
}

fn abc114_b() {
    let l: Vec<String> = read_line();
    let s = l[0].clone();

    let n = s.len();

    let mut ans: isize = 100000007;
    for ((i, j), k) in (0..n - 2).zip(1..n - 1).zip(2..n) {
        let cmp_num = abc114_b_f(
            s.chars().nth(i).unwrap(),
            s.chars().nth(j).unwrap(),
            s.chars().nth(k).unwrap(),
        );

        ans = cmp::min(ans, cmp_num as isize);
    }

    println!("{}", ans);
}

fn abc081_b() {
    let _l: Vec<usize> = read_line();
    let mut a: Vec<u128> = read_line();

    let mut cnt = 0;
    let mut tmp_v: Vec<u128> = Vec::new();
    loop {
        let mut flg = false;
        for e in a.iter() {
            if *e % 2 != 0 {
                flg = true;
                break;
            } else {
                tmp_v.push(*e / 2);
            }
        }

        if flg {
            break;
        }

        a = tmp_v.clone();
        tmp_v.clear();
        cnt += 1;
    }

    println!("{}", cnt);
}
