#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(unused_imports)]
use std::cmp;

#[allow(unused_imports)]
use proconio::input;

fn main() {
    abc046_b();
}

#[allow(dead_code)]
fn abc046_b() {
    input! {
        n: i32,
        k: i32,
    }

    let mut ans: i128 = 0;

    for i in 1..=n {
        if i == 1 {
            ans += k as i128;
        } else {
            ans *= (k - 1) as i128;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn agc004_a() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a % 2 == 0 || b % 2 == 0 || c % 2 == 0 {
        println!("0");
    } else {
        println!("{}", cmp::min(a * b, cmp::min(a * c, b * c)));
    }
}

#[allow(dead_code)]
fn abc121_c() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by(|a, b| a.0.partial_cmp(&(b.0)).unwrap());

    let mut ans: usize = 0;
    let mut cnt: usize = 0;

    for i in 0..n {
        if m - cnt >= ab[i].1 {
            ans += ab[i].0 * ab[i].1;
            cnt += ab[i].1;
            if cnt == m {
                println!("{}", ans);
                return;
            }
        } else {
            while ab[i].1 > 0 {
                ans += ab[i].0;
                ab[i].1 -= 1;
                cnt += 1;
                if cnt == m {
                    println!("{}", ans);
                    return;
                }
            }
        }
    }
}

#[warn(dead_code)]
fn code_festival_2017_qualc_b() {
    input! {
        n: u32,
        a: [i32; n],
    }

    let mut all = 1;
    let mut bad = 1;

    for i in 0..n {
        all *= 3;
        if a[i as usize] % 2 == 0 {
            bad *= 2;
        }
    }

    let ans = all - bad;
    println!("{}", ans);
}

#[warn(dead_code)]
fn abc093_c() {
    input! {
        mut v: [i32; 3],
    }

    let mut odd = 0;
    let mut even = 0;

    for i in 0..3 {
        if v[i] % 2 == 0 {
            even += 1
        } else {
            odd += 1;
        }
    }

    let mut ans = 0;

    if odd == 2 {
        ans += 1;
        for i in 0..3 {
            if v[i] % 2 == 1 {
                v[i] += 1;
            }
        }
    } else if even == 2 {
        ans += 1;
        for i in 0..3 {
            if v[i] % 2 == 0 {
                v[i] += 1;
            }
        }
    }

    let ma = cmp::max(v[0], cmp::max(v[1], v[2]));
    for i in 0..3 {
        ans += (ma - v[i]) / 2;
    }

    println!("{}", ans);
}

#[warn(dead_code)]
fn abc079_c() {
    input! {
        mut n: i32,
    }

    let d = n % 10;
    n /= 10;
    let c = n % 10;
    n /= 10;
    let b = n % 10;
    n /= 10;
    let a = n;

    let mut sum = a;
    //println!("{}{}{}{}")
    // a+b+c+d
    {
        sum += b;
        sum += c;
        sum += d;
        if sum == 7 {
            println!("{}+{}+{}+{}=7", a, b, c, d);
            return;
        }
    }
    sum = a;
    // a+b+c-d
    {
        sum += b;
        sum += c;
        sum -= d;
        if sum == 7 {
            println!("{}+{}+{}-{}=7", a, b, c, d);
            return;
        }
    }
    sum = a;
    // a+b-c+d
    {
        sum += b;
        sum -= c;
        sum += d;
        if sum == 7 {
            println!("{}+{}-{}+{}=7", a, b, c, d);
            return;
        }
    }
    sum = a;
    // a+b-c-d
    {
        sum += b;
        sum -= c;
        sum -= d;
        if sum == 7 {
            println!("{}+{}-{}-{}=7", a, b, c, d);
            return;
        }
    }
    sum = a;
    // a-b+c+d
    {
        sum -= b;
        sum += c;
        sum += d;
        if sum == 7 {
            println!("{}-{}+{}+{}=7", a, b, c, d);
            return;
        }
    }
    sum = a;
    // a-b+c-d
    {
        sum -= b;
        sum += c;
        sum -= d;
        if sum == 7 {
            println!("{}-{}+{}-{}=7", a, b, c, d);
            return;
        }
    }
    sum = a;
    // a-b-c+d
    {
        sum -= b;
        sum -= c;
        sum += d;
        if sum == 7 {
            println!("{}-{}-{}+{}=7", a, b, c, d);
            return;
        }
    }
    sum = a;
    // a-b-c-d
    {
        sum -= b;
        sum -= c;
        sum -= d;
        if sum == 7 {
            println!("{}-{}-{}-{}=7", a, b, c, d);
            return;
        }
    }
}
