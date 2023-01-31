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
    al2_5_4();
}

fn al2_1_3() {
    let l: Vec<usize> = read_line();

    println!("{}", l[0] * l[1] * l[2]);
}

fn al2_2_4() {
    let l: Vec<usize> = read_line();

    let n: usize = l[0];

    let l: Vec<usize> = read_line();

    let mut sum: usize = 0;
    for e in l.iter() {
        sum += e;
    }

    println!("{}", sum % 100);
}

fn al2_5_3() {
    let l: Vec<u128> = read_line();

    let n: u128 = l[0];

    let mut sum: u128 = 1;

    for i in 1..=n {
        sum *= i;
    }

    println!("{}", sum);
}

fn al2_5_4() {
    let l: Vec<usize> = read_line();

    let n: usize = l[0];

    for i in 2..=n {
        let mut cnt: usize = 0;
        for j in 1..=i {
            if i % j == 0 {
                cnt += 1;
            }
        }
        if cnt == 2 {
            print!("{} ", i);
        }
    }

    println!();
}
