use itertools::Itertools;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(unused_imports)]
use std::cmp;

#[allow(dead_code)]
fn read<T: FromStr>() -> T {
    let cin = stdin();
    let cin = cin.lock();
    let s: String = cin
        .bytes()
        .map(|c| c.expect("failed reading char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    s.parse().ok().expect("failed parsing")
}

fn main() {
    agc002_a();
}

#[allow(dead_code)]
fn agc002_a() {
    let a: i128 = read();
    let b: i128 = read();

    if a > 0 && b > 0 {
        println!("Positive");
    } else if a <= 0 && b >= 0 {
        println!("Zero");
    } else {
        let j_num = (a - b).abs();
        if j_num % 2 == 0 {
            println!("Negative");
        } else {
            println!("Positive");
        }
    }
}

#[allow(dead_code)]
fn abc134_c() {
    let n: usize = read();
    let mut v = Vec::new();

    for _ in 0..n {
        let a: usize = read();
        v.push(a);
    }

    let mut cmp_v = v.clone();
    cmp_v.sort();
    cmp_v.reverse();

    for i in 0..n {
        if v[i] == cmp_v[0] {
            println!("{}", cmp_v[1]);
        } else {
            println!("{}", cmp_v[0]);
        }
    }
}

#[allow(dead_code)]
fn abc123_b() {
    let a: isize = read();
    let b: isize = read();
    let c: isize = read();
    let d: isize = read();
    let e: isize = read();

    let v_l = [a, b, c, d, e];

    let mut ans: isize = 100000007;
    for perm in v_l.iter().permutations(5) {
        let mut sum: isize = 0;
        for i in 0..5 {
            if perm[i] % 10 != 0 && i != 4 {
                let quot = (perm[i] % 10 - 10).abs();
                let sum_quot_e = quot + perm[i];
                sum += sum_quot_e;
            } else {
                sum += perm[i];
            }
        }
        ans = cmp::min(ans, sum);
    }

    println!("{}", ans);
}
