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
#[allow(unused_imports)]
use std::{isize, usize};

fn main() {
    abc309_c();
}

#[allow(dead_code)]
fn abc309_c() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n],
    }

    let mut tot: usize = 0;
    for (_, b) in ab.clone() {
        tot += b;
    }

    if tot <= k {
        println!("1");
        return;
    }

    ab.sort_by(|a, b| a.0.cmp(&b.0));

    for (a, b) in ab {
        tot -= b;
        if tot <= k {
            println!("{}", a + 1);
            return;
        }
    }
}

#[allow(dead_code)]
fn abc309_b() {
    input! {
        n: usize,
        grid: [String; n],
    }

    let converted_grid: Vec<Vec<u32>> = grid
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let n = converted_grid.len();
    let m = converted_grid[0].len();

    let mut rotated_grid = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            if i == 0 {
                if j == 0 {
                    rotated_grid[i][j] = converted_grid[i + 1][j];
                } else {
                    rotated_grid[i][j] = converted_grid[i][j - 1];
                }
            } else if i == n - 1 {
                if j == m - 1 {
                    rotated_grid[i][j] = converted_grid[i - 1][j];
                } else {
                    rotated_grid[i][j] = converted_grid[i][j + 1];
                }
            } else if j == 0 {
                rotated_grid[i][j] = converted_grid[i + 1][j];
            } else if j == m - 1 {
                rotated_grid[i][j] = converted_grid[i - 1][j];
            } else {
                rotated_grid[i][j] = converted_grid[i][j];
            }
        }
    }

    for row in rotated_grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

#[allow(dead_code)]
fn abc309_a() {
    input! {
        a: i32,
        b: i32,
    }

    let vec1 = [1, 2, 3];
    let vec2 = [4, 5, 6];
    let vec3 = [7, 8, 9];

    if vec1.contains(&a) && vec1.contains(&b) {
        if a == 2 || b == 2 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if vec2.contains(&a) && vec2.contains(&b) {
        if a == 5 || b == 5 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if vec3.contains(&a) && vec3.contains(&b) {
        if a == 8 || b == 8 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn abc127_c() {
    input! {
        (n,m): (usize, usize),
        mut nd: [(isize, isize); m],
    }

    let (mut ll, mut rr) = (isize::MIN, isize::MAX);

    for (l, r) in nd {
        ll = cmp::max(ll, l);
        rr = cmp::min(rr, r);
        if ll > rr {
            break;
        }
    }

    println!("{}", cmp::max(0, rr - ll + 1));
}

#[allow(dead_code)]
fn code_festival_2016_qualc_b() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = 0;
    for i in 0..n {
        if a[a[i] - 1] == i + 1 {
            cnt += 1;
        }
    }

    println!("{}", cnt / 2);
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
