use std::fmt::Debug;
use std::str::FromStr;
use std::cmp::min;

fn read_line<T: std::str::FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}

fn main() {
    doublePoints();
}

fn doublePoints() {
    let S: Vec<String> = read_line();

    let str1: &str = &S[0];

    let changStr1 = str1.chars().map(|c| format!("{}{}", c, "``")).collect::<String>();

    println!("{}", changStr1);
}

fn abc156c() {
    let l: Vec<usize> = read_line();
    let N: usize = l[0];

    let x: Vec<isize> = read_line();

    let mut ans: isize = 1<<30;
    for p in 1..=100 {
        let mut cnt: isize = 0;
        for i in 0..N {
            cnt +=  (x[i] - p).pow(2);
        }
        ans = min(ans, cnt);
    }

    println!("{}", ans);
}

fn abc193_ans() {
    let l: Vec<u32> = read_line();

    let a: u32 = l[0];
    let b: u32 = l[1];

    let ans: u32 = (b-1+a-2)/(a-1);

    println!("{}", ans);
}

fn abc193() {
    let l: Vec<u32> = read_line();

    let a: u32 = l[0];
    let b: u32 = l[1];

    let mut ans: u32;

    let mut plug: u32 = 0;
    let mut cnt: u32 = 0;
    let max_num: u32 = 1000000007;
    for i in 1..=max_num {
        cnt += 1;
        ans = a*i-plug;
        if ans >= b {
            break;
        }
        plug += 1;
    }

    println!("{}", cnt);
}