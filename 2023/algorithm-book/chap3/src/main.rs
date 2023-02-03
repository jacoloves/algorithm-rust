use std::fmt::Debug;
use std::str::FromStr;
use std::u128;

fn read_line<T: std::str::FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}

fn main() {
    al3_2_2();
}

fn al3_1_2() {
    let l: Vec<u128> = read_line();
    let mut n: u128 = l[0];

    let mut flg: bool = false;
    for i in 2..n {
        if i * i > n {
            break;
        }
        while n % i == 0 {
            if flg == true {
                print!(" ");
            }
            flg = true;
            n /= i;
            print!("{}", i);
        }
    }
    if n >= 2 {
        if flg == true {
            print!(" ");
        }
        flg = true;
        print!("{}", n);
    }
    println!();
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while (a >= 1 && b >= 1) {
        if a < b {
            b = b % a;
        } else {
            a = a % b;
        }
    }
    if a >= 1 {
        a
    } else {
        b
    }
}

fn al3_2_2() {
    let l: Vec<u128> = read_line();
    let n = l[0];
    let l: Vec<u128> = read_line();

    let mut ans: u128 = gcd(l[0], l[1]);

    for i in 2..n {
        ans = gcd(ans, l[2]);
    }

    println!("{}", ans);
}

fn lcm(mut a: u128, mut b: u128) -> u128 {
    (a / gcd(a, b)) * b
}

fn al3_2_3() {
    let l: Vec<u128> = read_line();
    let n = l[0];
    let l: Vec<u128> = read_line();
}
