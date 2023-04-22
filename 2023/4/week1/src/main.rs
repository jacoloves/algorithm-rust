mod past;
use crate::past::week1::abc158_b;

use itertools::Itertools;
use std::cmp;
use std::cmp::min;
use std::fmt::Debug;
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
    abc114_b();
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
