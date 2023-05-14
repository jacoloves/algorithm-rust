use itertools::Itertools;
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(unused_imports)]
use std::cmp;

#[allow(dead_code)]
fn read_line<T: FromStr>() -> T {
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
    abc062_b();
}

#[allow(dead_code)]
fn abc062_b() {
    let h: usize = read_line();
    let w: usize = read_line();

    let mut a: Vec<String> = Vec::new();

    for _ in 1..=h {
        let s: String = read_line();
        a.push(s);
    }

    let mut ans: Vec<String> = Vec::new();

    let mut tmp_s = "".to_string();

    for _ in 1..=w + 2 {
        tmp_s.push('#');
    }

    for i in 0..h + 2 {
        if i == 0 || i == h + 1 {
            ans.push(tmp_s.clone());
        } else {
            let tmp_s2 = format!("#{}#", a[i - 1]);
            ans.push(tmp_s2);
        }
    }

    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}

#[allow(dead_code)]
fn abc139_c() {
    let n: u128 = read_line();
    let mut h: Vec<u128> = Vec::new();

    for _ in 1..=n {
        let tmp: u128 = read_line();
        h.push(tmp);
    }

    h.reverse();

    let mut ans = 0;
    let mut val = 0;
    for i in 1..n {
        if h[i as usize - 1] <= h[i as usize] {
            val += 1;
        } else {
            val = 0;
        }
        ans = cmp::max(ans, val);
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc079_b() {
    let n: usize = read_line();

    //let mut ans: u128 = 0;

    let mut v: Vec<u128> = Vec::new();

    v.push(2);
    v.push(1);

    for i in 2..=n {
        let tmp = v[i - 1] + v[i - 2];
        v.push(tmp);
    }

    println!("{}", v[v.len() - 1]);
}

#[allow(dead_code)]
fn agc024_a() {
    let a: i128 = read_line();
    let b: i128 = read_line();
    let _c: i128 = read_line();
    let k: i128 = read_line();

    const JUDGE_NUM: i128 = 1000000000000000000;

    if k % 2 == 0 {
        if (a - b).abs() >= JUDGE_NUM {
            println!("Unfair");
        } else {
            println!("{}", a - b);
        }
    } else {
        if (b - a).abs() >= JUDGE_NUM {
            println!("Unfair");
        } else {
            println!("{}", b - a);
        }
    }
}

#[allow(dead_code)]
fn abc090_b() {
    let a: usize = read_line();
    let b: usize = read_line();

    let mut ans = 0;
    for i in a..=b {
        let stri: String = i.to_string();
        let reverse: String = stri.chars().rev().collect::<String>();

        if stri == reverse {
            ans += 1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc124_c() {
    let s: String = read_line();

    // 0101...
    let mut cnt1 = 0;
    for (c, i) in s.chars().zip(0..s.len()) {
        if i % 2 == 0 {
            if c != '0' {
                cnt1 += 1;
            }
        } else {
            if c != '1' {
                cnt1 += 1;
            }
        }
    }
    // 1010...
    let mut cnt2 = 0;
    for (c, i) in s.chars().zip(0..s.len()) {
        if i % 2 == 0 {
            if c != '1' {
                cnt2 += 1;
            }
        } else {
            if c != '0' {
                cnt2 += 1;
            }
        }
    }

    println!("{}", cmp::min(cnt1, cnt2));
}

#[allow(dead_code)]
fn abc044_b() {
    let w: String = read_line();

    let mut map = HashMap::<char, u32>::new();

    for c in w.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let mut flg = true;
    for (_, value) in &map {
        if value % 2 != 0 {
            flg = false;
            break;
        }
    }

    if flg {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn abc118_b() {
    let n: usize = read_line();
    let m: usize = read_line();

    let mut v: Vec<usize> = Vec::new();

    for _ in 0..m {
        v.push(0);
    }

    for _ in 1..=n {
        let k: usize = read_line();
        for _ in 1..=k {
            let a: usize = read_line();
            v[a - 1] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..m {
        if v[i] == n {
            ans += 1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc152_c() {
    let n: usize = read_line();
    let mut p = Vec::new();

    for _ in 0..n {
        let a: usize = read_line();
        p.push(a);
    }

    let mut p_min = p[0];
    let mut p_max = p[0];

    let mut ans = 1;

    for i in 1..n {
        if p[i] <= p_max && p[i] <= p_min {
            ans += 1;
        }

        p_max = cmp::max(p[i], p_max);
        p_min = cmp::min(p[i], p_min);
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn agc002_a() {
    let a: i128 = read_line();
    let b: i128 = read_line();

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
    let n: usize = read_line();
    let mut v = Vec::new();

    for _ in 0..n {
        let a: usize = read_line();
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
    let a: isize = read_line();
    let b: isize = read_line();
    let c: isize = read_line();
    let d: isize = read_line();
    let e: isize = read_line();

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
