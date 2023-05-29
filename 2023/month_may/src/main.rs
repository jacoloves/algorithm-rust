use itertools::Itertools;
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
use proconio::marker::Chars;

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
    abc058_b();
}

#[allow(dead_code)]
fn abc058_b() {
    input! {
        o: String,
        e: String,
    }

    let mut a: String = "".to_string();

    let diff = o.len() - e.len();

    for (i, j) in o.chars().zip(e.chars()) {
        a.push(i);
        a.push(j);
    }

    if diff == 1 {
        a.push(o.chars().nth(o.len() - 1).unwrap());
    }

    println!("{}", a);
}

#[allow(dead_code)]
fn abc148_d() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut cnt = 1;
    for e in a.iter() {
        if *e as isize != cnt {
            ans += 1;
        } else {
            cnt += 1;
        }
    }

    if ans == n {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn abc049_b() {
    input! {
        h: usize,
        _w: usize,
        c: [String;h],
    }

    for e in c.iter() {
        for _ in 1..=2 {
            println!("{}", e);
        }
    }
}

#[allow(dead_code)]
fn abc103_b() {
    input! {
        s: String,
        t: String,
    }

    let len: usize = s.len();

    for i in 0..len {
        if format!("{}{}", &s[len - i..len], &s[0..len - i]) == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

#[allow(dead_code)]
fn abc050_b() {
    input! {
        n: usize,
        t: [usize; n],
        m: usize,
        p: [[usize; 2]; m],
    }

    for i in 0..p.len() {
        let mut tmp_t = t.clone();
        tmp_t[p[i][0] - 1] = p[i][1];
        let ans: usize = tmp_t.iter().sum();
        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn abc104_b() {
    input! {
        s: String,
    };

    let mut cnt = 0;
    let mut c_cnt = 0;

    for c in s.chars() {
        if cnt == 0 && c != 'A' {
            println!("WA");
            return;
        }

        if cnt == 1 && c.is_uppercase() {
            println!("WA");
            return;
        }

        if cnt >= 2 && cnt != s.len() - 1 && c.is_uppercase() && c_cnt == 0 {
            c_cnt += 1;
        } else if cnt >= 2 && c_cnt >= 1 && c.is_uppercase() {
            println!("WA");
            return;
        }

        if cnt == s.len() - 1 && c.is_uppercase() {
            println!("WA");
            return;
        }

        cnt += 1;
    }

    if c_cnt == 0 {
        println!("WA");
        return;
    }

    println!("AC");
}

#[allow(dead_code)]
fn abc113_b_ans() {
    input! {
        n: usize,
        t: f64,
        a: f64,
        h: [f64; n],
    };

    let mut p = 200000.0;
    let mut r = 0;
    for i in 0..n {
        let q = t - h[i] * 0.006;
        let d = (a - q).abs();
        if d < p {
            p = d;
            r = i;
        }
    }

    println!("{}", r + 1);
}

#[allow(dead_code)]
fn abc113_b() {
    input! {
        n: usize,
        t: i32,
        a: i32,
        h: [i32; n],
    }

    let mut v = vec![];

    for e in h.iter() {
        let tmp = t as f32 - *e as f32 * 0.006;
        v.push(tmp);
    }

    let mut map = HashMap::new();
    for (e, i) in v.iter().zip(1..=v.len()) {
        let cmp_e = (a - *e as i32).abs();
        map.insert(i, cmp_e);
    }

    let mut ans_v: Vec<(&usize, &i32)> = map.iter().collect();
    ans_v.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{}", ans_v[0].0);
}

#[allow(dead_code)]
fn abc158_c() {
    input! {
        a: usize,
        b: usize,
    }

    for i in 1..=10001 {
        let tmp_a = (i as f64 * 0.08) as usize;
        let tmp_b = (i as f64 * 0.1) as usize;
        if a == tmp_a && b == tmp_b {
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}

#[allow(dead_code)]
fn abc093_b() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    let mut ans = vec![];

    for i in 0..k {
        let c = a + i;

        if c > b {
            break;
        }
        ans.push(c);
    }

    for i in 0..k {
        let d = b - i;

        if d < a {
            break;
        }
        ans.push(d);
    }

    ans.sort();
    ans.dedup();

    for e in ans.iter() {
        println!("{}", e);
    }
}

#[allow(dead_code)]
fn abc302_b() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let directions = [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    let snuke = ['s', 'n', 'u', 'k', 'e'];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == snuke[0] {
                for &(vy, vx) in &directions {
                    let mut y = i as i64;
                    let mut x = j as i64;
                    let mut ans = vec![];
                    ans.push((y, x));

                    for k in 1..snuke.len() {
                        y = y + vy;
                        x = x + vx;
                        if x < 0 || y < 0 || x >= w as i64 || y >= h as i64 {
                            break;
                        }
                        if s[y as usize][x as usize] == snuke[k] {
                            ans.push((y, x));
                        } else {
                            break;
                        }
                    }
                    if ans.len() == snuke.len() {
                        for &(y, x) in &ans {
                            println!("{} {}", y + 1, x + 1);
                        }
                        return;
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn abc302_a() {
    let a: i128 = read_line();
    let b: i128 = read_line();

    let mut ans = a / b;
    if a % b != 0 {
        ans += 1;
    }
    println!("{}", ans);
}

#[allow(dead_code)]
fn abc133_b() {
    let n: usize = read_line();
    let d: usize = read_line();

    let mut x: Vec<Vec<i32>> = vec![];

    for _ in 1..=n {
        let mut y = vec![];
        for _ in 1..=d {
            let a: i32 = read_line();
            y.push(a);
        }
        x.push(y);
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let mut distance = 0;
            for k in 0..d {
                distance += (x[i][k] - x[j][k]).pow(2);
            }
            let s = (distance as f32).sqrt();
            if s.fract() == 0.0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

#[allow(dead_code)]
fn abc109_b_ex() {
    let n: usize = read_line();

    let mut w: Vec<String> = Vec::new();
    for _ in 1..=n {
        let w_c: String = read_line();
        w.push(w_c);
    }

    let mut map_w = HashMap::new();
    for i in w.iter() {
        *map_w.entry(i).or_insert(0) += 1;
    }

    for i in map_w.into_values() {
        if i >= 2 {
            println!("No");
            return;
        }
    }
}

#[allow(dead_code)]
fn abc109_b() {
    let n: usize = read_line();

    let mut w: Vec<String> = Vec::new();

    for _ in 1..=n {
        let w_c: String = read_line();
        w.push(w_c);
    }

    for i in 0..n {
        for j in i + 1..n {
            if w[i] == w[j] {
                println!("No");
                return;
            }
        }
    }

    let mut tmp_w = w[0].clone();

    for i in 1..n {
        if tmp_w.chars().last().unwrap() != w[i].clone().chars().next().unwrap() {
            println!("No");
            return;
        }
        tmp_w = w[i].clone();
    }

    println!("Yes");
}

#[allow(dead_code)]
fn abc061_b() {
    let n: usize = read_line();
    let m: usize = read_line();

    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();

    for _ in 1..=m {
        let tmp_a: usize = read_line();
        let tmp_b: usize = read_line();

        a.push(tmp_a);
        b.push(tmp_b);
    }

    let mut ans: Vec<usize> = Vec::new();
    for _ in 1..=n {
        ans.push(0);
    }

    for (a_e, b_e) in a.iter().zip(b.iter()) {
        ans[a_e - 1] += 1;
        ans[b_e - 1] += 1;
    }

    for e in ans.iter() {
        println!("{}", e);
    }
}

#[allow(dead_code)]
fn abc140_c() {
    let n: usize = read_line();
    let mut b: Vec<usize> = Vec::new();

    for _ in 1..=n - 1 {
        let a: usize = read_line();
        b.push(a);
    }

    let mut ans: Vec<usize> = Vec::new();

    ans.push(b[0]);

    for i in 1..n {
        ans.push(b[i - 1]);
        if !(b[i - 1] >= cmp::max(ans[i - 1], ans[i])) {
            ans[i - 1] = b[i - 1];
        }
    }

    let ans_sum: usize = ans.iter().sum();

    println!("{}", ans_sum);
}

#[allow(dead_code)]
fn abc083_b() {
    let n: usize = read_line();
    let a: usize = read_line();
    let b: usize = read_line();

    let mut ans = 0;
    for i in 1..=n {
        let mut tmp_sum = 0;
        let mut tmp = i;
        while tmp != 0 {
            tmp_sum += tmp % 10;
            tmp /= 10;
        }

        if tmp_sum >= a && tmp_sum <= b {
            ans += i;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc059_b() {
    let a: String = read_line();
    let b: String = read_line();

    if a.len() > b.len() {
        println!("GREATER");
    } else if a.len() < b.len() {
        println!("LESS");
    } else {
        for (a_c, b_c) in a.chars().zip(b.chars()) {
            let num_a_c = a_c as i32 - 48;
            let num_b_c = b_c as i32 - 48;
            if num_a_c > num_b_c {
                println!("GREATER");
                return;
            } else if num_a_c < num_b_c {
                println!("LESS");
                return;
            }
        }
        println!("EQUAL");
    }
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
