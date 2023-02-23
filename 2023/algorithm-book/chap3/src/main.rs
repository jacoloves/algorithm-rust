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

fn al3_1_2() {
    let l: Vec<u128> = read_line();
    let mut n: u128 = l[0];

    let mut flg: bool = false;
    for i in 2..n {
        if i * i > n {
            break;
        }
        while n % i == 0 {
            if flg {
                print!(" ");
            }
            flg = true;
            n /= i;
            print!("{}", i);
        }
    }
    if n >= 2 {
        if flg {
            print!(" ");
        }
        print!("{}", n);
    }
    println!();
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while a >= 1 && b >= 1 {
        if a < b {
            b %= a;
        } else {
            a %= b;
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

    let mut r: u128 = lcm(l[0], l[1]);

    for i in 2..n {
        r = lcm(r, l[i as usize]);
    }

    println!("{}", r);
}

fn al3_3_3() {
    let l: Vec<usize> = read_line();
    let mut n: usize = l[0];
    let r: usize = l[1];

    let mut rtmp: u128 = 1;
    let mut ntmp: u128 = 1;

    for _ in 1..=r {
        rtmp *= n as u128;
        n -= 1;
    }

    for i in 1..=r {
        ntmp *= i as u128;
    }

    println!("{}", rtmp/ntmp);
}

fn al3_3_4() {
    let l: Vec<usize> = read_line();
    let _: usize = l[0];
    let l: Vec<u128> = read_line();

    let mut a: usize = 0;
    let mut b: usize = 0;
    let mut c: usize = 0;
    let mut d: usize = 0;

    for i in l.iter() {
        if *i == 100 {
            a += 1; 
        } else if *i == 200 {
            b += 1;
        } else if *i == 300 {
            c += 1;
        } else if *i == 400 {
            d += 1;
        }
    }

    let ad: u128 = a as u128 * d as u128;
    let bc: u128 = b as u128 * c as u128;

    println!("{}", ad+bc);
}

fn al3_3_5() {
    let l: Vec<usize> = read_line();
    let _: usize = l[0];
    let l: Vec<u128> = read_line();

    let mut a: u128 = 0;
    let mut b: u128 = 0;
    let mut c: u128 = 0;

    for i in l.iter() {
        if *i == 1 {
            a += 1; 
        } else if *i == 2 {
            b += 1;
        } else if *i == 3 {
            c += 1;
        }
    }

    let atmp: u128 = a*(a-1)/2;
    let btmp: u128 = b*(b-1)/2;
    let ctmp: u128 = c*(c-1)/2;

    println!("{}", atmp+btmp+ctmp);
}

fn al3_3_6() {
    let l: Vec<usize> = read_line();
    let _: usize = l[0];
    let l: Vec<u128> = read_line();

    let mut cnt: Vec<u128> = vec![0; 100000];
    
    for i in l.iter() {
        cnt[*i as usize] += 1;
    }

    let mut answer: u128 = 0;

    for i in 0..49999 {
        answer += cnt[i] * cnt[99999-i];
    }
    answer += cnt[50000] * (cnt[50000]-1)/2;

    println!("{}", answer);
}

fn al3_4_3() {
    let l: Vec<usize> = read_line();
    let n: usize = l[0];

    let a: Vec<f64> = read_line();
    let b: Vec<f64> = read_line();

    let mut sum: f64 = 0.0;

    for i in 0..n {
        sum += a[i] * 1.0/3.0 + b[i] * 2.0/3.0;
    }

    println!("{}", sum);

}

fn main() {
    al3_4_3();
}