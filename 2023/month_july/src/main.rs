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
    abc093_c();
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
