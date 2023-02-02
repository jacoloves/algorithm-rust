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
    al3_1_2();
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
