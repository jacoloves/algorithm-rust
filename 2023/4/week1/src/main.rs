use std::fmt::Debug;
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
    abc193_ans();
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