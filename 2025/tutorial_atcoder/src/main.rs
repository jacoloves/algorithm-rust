#[allow(unused_imports)]
use itertools::min;
#[allow(unused_imports)]
use proconio::input;
/* ↓aoj */
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

#[allow(unused_imports)]
use std::isize;
#[allow(unused_imports)]
use std::iter::zip;
use std::usize;

#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
use std::str::FromStr;

#[allow(dead_code)]
fn read_line<T: std::str::FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}
/* ↑AOJ */

const MOD: usize = 1_000_000_000;

fn main() {
    abc407d();
}

#[allow(dead_code)]
// https://atcoder.jp/contests/abc407/tasks/abc407_d
fn abc407d_sai() {
    input! {
        h: usize,
        w: usize,
    }

    let n = h * w;
    let mut a = Vec::with_capacity(n);
    for _ in 0..h {
        input! {
            row: [u64; w]
        }
        a.extend_from_slice(&row);
    }

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..h {
        for j in 0..w {
            let v = i * w + j;
            if j + 1 < w {
                let r = v + 1;
                adj[v].push(r);
                adj[r].push(v);
            }
            if i + 1 < h {
                let d = v + w;
                adj[v].push(d);
                adj[d].push(v);
            }
        }
    }

    let full: u32 = (1u32 << n) - 1;
    let size = 1usize << n;

    let mut dp = vec![false; size];
    dp[0] = true;
    for mask in 1u32..=full {
        let i = mask.trailing_zeros() as usize;
        let mut ok = false;
        for &j in &adj[i] {
            if mask & (1u32 << j) != 0 {
                let next = mask ^ (1u32 << i) ^ (1u32 << j);
                if dp[next as usize] {
                    ok = true;
                    break;
                }
            }
        }
        dp[mask as usize] = ok;
    }

    let mut xor = vec![0u64; size];
    for mask in 1u32..=full {
        let b = mask.trailing_zeros() as usize;
        xor[mask as usize] = xor[(mask ^ (1u32 << b)) as usize] ^ a[b];
    }

    let mut ans = 0u64;
    for mask in 0u32..=full {
        if dp[(full ^ mask) as usize] {
            ans = ans.max(xor[mask as usize]);
        }
    }

    println!("{}", ans);

}

#[allow(dead_code)]
// https://atcoder.jp/contests/abc407/tasks/abc407_c
fn abc407c_sai() {
    input! {
        s: String
    }

    let digits: Vec<i64> = s.bytes().map(|b| (b - b'0') as i64).collect();
    let n = digits.len() as i64;

    let mut last = *digits.last().unwrap();
    for &d in digits[..digits.len() - 1].iter().rev() {
        let diff = (d - (last % 10) + 10) % 10;
        last += diff;
    }

    let answer = n + last;
    println!("{}", answer);
}

#[allow(dead_code)]
// https://atcoder.jp/contests/abc407/tasks/abc407_b
fn abc407b_sai() {
    input! {
        x: usize,
        y: usize,
    }

    let mut condition_cnt = 0;

    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || (i as isize - j as isize).abs() >= y as isize {
                condition_cnt += 1;
            }
        }
    }

    let probability = condition_cnt as f64 / 36.0;

    println!("{}", probability);
}

#[allow(dead_code)]
// https://atcoder.jp/contests/abc407/tasks/abc407_a
fn abc407a_sai() {
    input! {
        a: usize,
        b: usize
    }

    let ans = (a + b / 2) / b;

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc407d() {
    input! {
        h: usize,
        w: usize,
    }

    let n = h * w;
    let mut a = Vec::with_capacity(n);
    for _ in 0..h {
        input! {
            row: [u64; w]
        }
        a.extend_from_slice(&row);
    }

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..h {
        for j in 0..w {
            let v = i * w + j;
            if j + 1 < w {
                let r = v + 1;
                adj[v].push(r);
                adj[r].push(v);
            }
            if i + 1 < h {
                let d = v + w;
                adj[v].push(d);
                adj[d].push(v);
            }
        }
    }

    let full: u32 = (1u32 << n) - 1;
    let size = 1usize << n;

    let mut dp = vec![false; size];
    dp[0] = true;
    for mask in 1u32..=full {
        let i = mask.trailing_zeros() as usize;
        let mut ok = false;
        for &j in &adj[i] {
            if mask & (1u32 << j) != 0 {
                let next = mask ^ (1u32 << i) ^ (1u32 << j);
                if dp[next as usize] {
                    ok = true;
                    break;
                }
            }
        }
        dp[mask as usize] = ok;
    }

    let mut xor = vec![0u64; size];
    for mask in 1u32..=full {
        let b = mask.trailing_zeros() as usize;
        xor[mask as usize] = xor[(mask ^ (1u32 << b)) as usize] ^ a[b];
    }

    let mut ans = 0u64;
    for mask in 0u32..=full {
        if dp[(full ^ mask) as usize] {
            ans = ans.max(xor[mask as usize]);
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc407c() {
    input! {
        s: String
    }

    let digits: Vec<i64> = s.bytes().map(|b| (b - b'0') as i64).collect();
    let n = digits.len() as i64;

    let mut last = *digits.last().unwrap();
    for &d in digits[..digits.len() - 1].iter().rev() {
        let diff = (d - (last % 10) + 10) % 10;
        last += diff;
    }

    let answer = n + last;
    println!("{}", answer);
}

#[allow(dead_code)]
fn abc407b() {
    input! {
        x: usize,
        y: usize,
    }

    let mut codition_cnt = 0;
    // i + j >= x
    // (i - j).abs() >= y
    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || (i as isize - j as isize).abs() >= y as isize {
                codition_cnt += 1;
            }
        }
    }

    let probability = codition_cnt as f64 / 36.0;

    println!("{}", probability);
}

#[allow(dead_code)]
fn abc407a() {
    input! {
        a: usize,
        b: usize
    }

    let ans = (a + b / 2) / b;

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc104b() {
    input! {
        s: String,
    }

    let mut c_cnt = 0;
    let mut len_cnt = 0;
    let mut upper_cnt = 0;

    for c in s.chars() {
        len_cnt += 1;
        if c != 'A' && len_cnt == 1 {
            println!("WA");
            return;
        }
        if c == 'C' && len_cnt >= 3 {
            c_cnt += 1;
        }
        if c_cnt > 1 {
            println!("WA");
            return;
        }

        if c == 'C' && len_cnt == s.len() {
            println!("WA");
            return;
        }

        if c.is_uppercase() {
            upper_cnt += 1;
        }
    }

    if c_cnt == 1 && upper_cnt == 2 {
        println!("AC");
    } else {
        println!("WA");
    }
}

#[allow(dead_code)]
fn tenkei076() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let total: usize = a.iter().sum();
    if total % 10 != 0 {
        println!("No");
        return;
    }
    let target = total / 10;
    let mut sum = 0;
    let mut r = 0;

    // l: left pointer
    for l in 0..n {
        while r < l + n && sum < target {
            sum += a[r % n];
            r += 1;
        }
        if sum == target {
            println!("Yes");
            return;
        }
        sum -= a[l];
    }
    println!("No");
}

#[allow(dead_code)]
fn abc113c() {
    input! {
        n: usize,
        m: usize,
        pairs: [(usize, usize); m],
    }

    let mut city_info = Vec::with_capacity(m);
    for (i, &(p, y)) in pairs.iter().enumerate() {
        city_info.push((p, y, i));
    }

    city_info.sort_by_key(|&(_p, y, _i)| y);

    let mut cnt = vec![0; n + 1];
    let mut ans = vec![String::new(); m];

    for &(p, _y, orig_idx) in city_info.iter() {
        cnt[p] += 1;
        ans[orig_idx] = format!("{:06}{:06}", p, cnt[p]);
    }

    for code in ans {
        println!("{}", code);
    }
}

#[allow(dead_code)]
fn abc121c() {
    input! {
        n: usize,
        m: usize,
        mut pairs: [(usize, usize); n],
    }

    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let mut sum_a = 0;
    let mut remain_m = m;

    for i in 0..n {
        if remain_m == 0 {
            break;
        }

        if remain_m > pairs[i].1 {
            sum_a += pairs[i].0 * pairs[i].1;
            remain_m -= pairs[i].1;
        } else {
            sum_a += pairs[i].0 * remain_m;
            remain_m = 0;
        }
    }

    println!("{}", sum_a);
}

#[allow(dead_code)]
fn abc115c() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    h.sort();

    let mut min_diff = usize::MAX;

    for i in 0..=n - k {
        let diff = h[i + k - 1] - h[i];
        if diff < min_diff {
            min_diff = diff;
        }
    }

    println!("{}", min_diff);
}

#[allow(dead_code)]
fn abc132c() {
    input! {
        n: usize,
        mut d: [usize; n],
    }

    d.sort();

    let mid = n / 2;
    if d[mid] > d[mid - 1] {
        println!("{}", d[mid] - d[mid - 1]);
    } else {
        println!("0");
    }
}

#[allow(dead_code)]
fn abc067b() {
    input! {
        n: usize,
        k: usize,
        mut l: [usize; n],
    }

    // sort l in descending order
    l.sort_by(|a, b| b.cmp(a));

    let mut sum = 0;
    for i in 0..k {
        sum += l[i];
    }

    println!("{}", sum);
}

#[allow(dead_code)]
fn abc205b() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // elemnt count
    let mut cnt = vec![0; n];

    for i in 0..n {
        cnt[a[i] - 1] += 1;
    }

    for i in 0..n {
        if cnt[i] > 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

#[allow(dead_code)]
fn abc113b() {
    input! {
        n: usize,
        t: f64,
        a: f64,
        h: [f64; n]
    }

    let mut closest_index = 0;
    let mut min_diff = f64::MAX;

    for i in 0..n {
        let temp = t - h[i] * 0.006;
        let diff = (temp - a).abs();

        if diff < min_diff {
            min_diff = diff;
            closest_index = i;
        }
    }

    println!("{}", closest_index + 1);
}

#[allow(dead_code)]
fn abc404d() {
    input! {
        n: usize,
        m: usize,
        c: [i64; n],
    }

    let mut animals_in_zoo: Vec<Vec<usize>> = vec![Vec::new(); n];

    for animal in 0..m {
        input! {
            k: usize,
            a: [usize; k]
        }
        for &z in &a {
            animals_in_zoo[z - 1].push(animal);
        }
    }

    let states = 3usize.pow(n as u32);
    let mut best: Option<i64> = None;

    for mut st in 0..states {
        let mut cost: i64 = 0;
        let mut cnt = vec![0u8; m];

        for zoo in 0..n {
            let visits = st % 3;
            st /= 3;

            if visits == 0 {
                continue;
            }

            cost += (visits as i64) * c[zoo];

            for &animal in &animals_in_zoo[zoo] {
                cnt[animal] += visits as u8;
            }
        }

        if cnt.iter().all(|&x| x >= 2) {
            best = Some(best.map_or(cost, |cur| cur.min(cost)));
        }
    }

    println!("{}", best.unwrap_or(-1));
}

#[allow(dead_code)]
fn abc404c() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize
        }
        let (a, b) = (a - 1, b - 1);
        g[a].push(b);
        g[b].push(a);
    }

    if m != n {
        println!("No");
        return;
    }

    if g.iter().any(|v| v.len() != 2) {
        println!("No");
        return;
    }

    let mut visited = vec![false; n];
    let mut que = VecDeque::new();
    visited[0] = true;
    que.push_back(0);

    while let Some(v) = que.pop_front() {
        for &u in &g[v] {
            if !visited[u] {
                visited[u] = true;
                que.push_back(u);
            }
        }
    }

    if visited.iter().all(|&x| x) {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn rotate90(src: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = src.len();
    let mut dst = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            dst[j][n - 1 - i] = src[i][j];
        }
    }
    dst
}

#[allow(dead_code)]
fn diff(a: &[Vec<char>], b: &[Vec<char>]) -> usize {
    let n = a.len();
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                cnt += 1;
            }
        }
    }
    cnt
}

#[allow(dead_code)]
fn abc404b() {
    input! {
        n: usize,
    }

    let mut s: Vec<Vec<char>> = (0..n)
        .map(|_| {
            input! { row: String }
            row.chars().collect()
        })
        .collect();

    let t: Vec<Vec<char>> = (0..n)
        .map(|_| {
            input! { row: String }
            row.chars().collect()
        })
        .collect();

    let mut best = usize::MAX;
    for rot in 0..4 {
        best = best.min(diff(&s, &t) + rot);
        s = rotate90(&s);
    }

    println!("{}", best);
}

#[allow(dead_code)]
fn abc404a() {
    input! {
        s: String
    }

    let mut seen = [false; 26];
    for c in s.chars() {
        seen[(c as u8 - b'a') as usize] = true;
    }

    for i in 0..26 {
        if !seen[i] {
            println!("{}", (b'a' + i as u8) as char);
            return;
        }
    }
    abc201b();
}

#[allow(dead_code)]
fn abc201b() {
    input! {
        n: usize,
        mut pair: [(String, usize); n],
    }

    // sort by second element descending order
    pair.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{}", pair[1].0);
}

#[allow(dead_code)]
fn abc124b() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut cnt = 1;
    let first_a = a[0];
    let mut max_a = a[0];

    for i in 1..n {
        if first_a <= a[i] && a[i - 1] <= a[i] && max_a <= a[i] {
            max_a = a[i];
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn operate(n: u64) -> u64 {
    let mut x = n;
    let mut digits = Vec::new();
    if x == 0 {
        digits.push('0');
    }

    while x > 0 {
        let d = (x % 9) as u32;
        digits.push(std::char::from_digit(d, 10).unwrap());
        x /= 9;
    }
    digits.reverse();
    let base9 = digits.into_iter().collect::<String>();

    let replaced = base9
        .chars()
        .map(|c| if c == '8' { '5' } else { c })
        .collect::<String>();

    u64::from_str_radix(&replaced, 8).unwrap()
}

#[allow(dead_code)]
fn tenkei067() {
    input! {
        n_str: String,
        k: usize
    }

    let mut n = u64::from_str_radix(&n_str, 8).unwrap();

    for _ in 0..k {
        n = operate(n);
    }

    println!("{:o}", n);
    abc095b();
}

#[allow(dead_code)]
fn abc095b() {
    input! {
        n: usize,
        mut x: usize,
        mut a: [usize; n]
    }

    a.sort();
    let a_min = a[0];

    let mut cnt = n;

    for elem in a {
        x -= elem;
    }

    loop {
        if x < a_min {
            break;
        }

        x -= a_min;
        cnt += 1;
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn abc130b() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n]
    }

    let mut d = 0;
    let mut ans = 0;

    for i in 0..n {
        if d <= x {
            ans += 1;
        }

        d += a[i];
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc174b() {
    input! {
        n: usize,
        d: usize,
        xy: [(isize, isize); n]
    }

    let mut ans = 0;

    for i in 0..n {
        let sq = (xy[i].0 * xy[i].0 + xy[i].1 * xy[i].1) as f64;
        let sq_root = sq.sqrt();
        if sq_root <= d as f64 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc088b() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut alice = 0;
    let mut bob = 0;

    a.sort_by(|a, b| b.cmp(a));

    for i in 0..n {
        if i % 2 == 0 {
            alice += a[i];
        } else {
            bob += a[i];
        }
    }

    println!("{}", alice - bob);
}

#[allow(dead_code)]
fn abc081b() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut dev_cnt = 0;

    loop {
        let mut flg = true;
        for i in 0..n {
            if a[i] % 2 != 0 {
                flg = false;
                break;
            }
        }
        if flg {
            for i in 0..n {
                a[i] /= 2;
            }
            dev_cnt += 1;
        } else {
            break;
        }
    }

    println!("{}", dev_cnt);
}

#[allow(dead_code)]
fn abc220b() {
    input! {
        k: usize,
        mut a: usize,
        mut b: usize
    }

    let mut i = 0;
    let mut a_decimal = 0;
    let mut b_decimal = 0;
    while a > 0 {
        let a_tmp = a % 10;
        a_decimal += a_tmp * k.pow(i as u32);
        a /= 10;
        i += 1;
    }

    i = 0;
    while b > 0 {
        let b_tmp = b % 10;
        b_decimal += b_tmp * k.pow(i as u32);
        b /= 10;
        i += 1;
    }

    let ans = a_decimal * b_decimal;
    println!("{}", ans);
}

#[allow(dead_code)]
fn abc090b() {
    input! {
        a: usize,
        b: usize
    }

    let mut palindrome_number = 0;

    for i in a..=b {
        let s = i.to_string();
        let s_rev = s.chars().rev().collect::<String>();
        if s == s_rev {
            palindrome_number += 1;
        }
    }

    println!("{}", palindrome_number);
}

#[allow(dead_code)]
fn abc068b() {
    input! {
        n: usize
    }

    let mut div_max_cnt = 0;
    let mut div_max = 1;

    for i in 1..=n {
        let mut div_i = i;
        let mut div_cnt = 0;
        while div_i / 2 != 0 {
            div_i /= 2;
            div_cnt += 1;
        }

        if div_cnt > div_max_cnt {
            div_max_cnt = div_cnt;
            div_max = i;
        }
    }

    println!("{}", div_max);
}

#[allow(dead_code)]
fn abc158c() {
    input! {
        a: usize,
        b: usize
    }

    let mut money: f64 = 1_f64;
    loop {
        let a_tax = money * 0.08;
        let b_tax = money * 0.1;

        let a_tax = a_tax.floor() as usize;
        let b_tax = b_tax.floor() as usize;

        if a_tax == a && b_tax == b {
            println!("{}", money);
            return;
        }

        if a_tax > a || b_tax > b {
            println!("-1");
            return;
        }

        money += 1_f64;
    }
}

#[allow(dead_code)]
fn abc093b() {
    input! {
        a: usize,
        b: usize,
        k: usize
    }

    if (b - a + 1) <= k {
        for i in a..=b {
            println!("{}", i);
        }
        return;
    }

    let mut sub_vec = vec![];
    let mut cnt = 0;
    for i in a..=b {
        if cnt < k {
            sub_vec.push(i);
            cnt += 1;
        } else {
            break;
        }
    }

    cnt = 0;
    for i in (a..=b).rev() {
        if cnt < k {
            sub_vec.push(i);
            cnt += 1;
        } else {
            break;
        }
    }

    sub_vec.sort();
    sub_vec.dedup();

    for i in sub_vec {
        println!("{}", i);
    }
}

#[allow(dead_code)]
fn abc208b() {
    input! {
        mut p: usize
    }

    let factorial_10 = (1..=10).fold(1, |acc, x| acc * x);
    let factorial_9 = (1..=9).fold(1, |acc, x| acc * x);
    let factorial_8 = (1..=8).fold(1, |acc, x| acc * x);
    let factorial_7 = (1..=7).fold(1, |acc, x| acc * x);
    let factorial_6 = (1..=6).fold(1, |acc, x| acc * x);
    let factorial_5 = (1..=5).fold(1, |acc, x| acc * x);
    let factorial_4 = (1..=4).fold(1, |acc, x| acc * x);
    let factorial_3 = (1..=3).fold(1, |acc, x| acc * x);
    let factorial_2 = (1..=2).fold(1, |acc, x| acc * x);
    let factorial_1 = (1..=1).fold(1, |acc, x| acc * x);

    let mut cnt = 0;

    loop {
        if p >= factorial_10 {
            p -= factorial_10;
            cnt += 1;
        } else if p >= factorial_9 {
            p -= factorial_9;
            cnt += 1;
        } else if p >= factorial_8 {
            p -= factorial_8;
            cnt += 1;
        } else if p >= factorial_7 {
            p -= factorial_7;
            cnt += 1;
        } else if p >= factorial_6 {
            p -= factorial_6;
            cnt += 1;
        } else if p >= factorial_5 {
            p -= factorial_5;
            cnt += 1;
        } else if p >= factorial_4 {
            p -= factorial_4;
            cnt += 1;
        } else if p >= factorial_3 {
            p -= factorial_3;
            cnt += 1;
        } else if p >= factorial_2 {
            p -= factorial_2;
            cnt += 1;
        } else if p >= factorial_1 {
            p -= factorial_1;
            cnt += 1;
        } else {
            break;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn abc164b() {
    input! {
        mut a: usize,
        b: usize,
        mut c: usize,
        d: usize
    }

    loop {
        // hero turn
        c = c.saturating_sub(b);
        if c == 0 {
            println!("Yes");
            return;
        }
        // monstaer turn
        a = a.saturating_sub(d);
        if a == 0 {
            println!("No");
            return;
        }
    }
}

#[allow(dead_code)]
fn abc200b() {
    input! {
        mut n: usize,
        k: usize
    }

    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            n = n * 1000 + 200;
        }
    }

    println!("{}", n);
}

#[allow(dead_code)]
fn abc165b() {
    input! {
        x: usize,
    }

    let mut ans = 100;
    let mut i = 1;

    while ans < x {
        ans += ans / 100;
        i += 1;
    }

    println!("{}", i - 1);
}

#[allow(dead_code)]
fn abc206b() {
    input! {
        n: usize
    }

    let mut sum = 1;

    let mut i = 1;
    while n >= sum {
        sum += i;
        i += 1;
    }

    println!("{}", i - 1);
}

#[allow(dead_code)]
fn abc162b() {
    input! {
        n: usize,
    }

    let mut sum = 0;

    for i in 1..=n {
        if i % 3 == 0 || i % 5 == 0 {
            continue;
        }
        sum += i;
    }

    println!("{}", sum);
}

#[allow(dead_code)]
fn abc153a() {
    input! {
        h: usize,
        a: usize,
    }

    if h < a {
        println!("1");
        return;
    }

    let mut ans = h / a;

    if h % a != 0 {
        ans += 1;
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn dfs(idx: usize, k: usize, n: usize, s: &[u8], pos: &mut Vec<usize>, all: &mut Vec<Vec<usize>>) {
    if pos.len() == k {
        all.push(pos.clone());
        return;
    }
    if idx >= n {
        return;
    }

    dfs(idx + 1, k, n, s, pos, all);

    if s[idx] == b'.' {
        return;
    }
    if !pos.is_empty() && idx <= pos[pos.len() - 1] + 1 {
        return;
    }

    pos.push(idx);
    dfs(idx + 1, k, n, s, pos, all);
    pos.pop();
}

#[allow(dead_code)]
// fn abc401d() {
//     const DOT: u8 = b'.';
//     const O: u8 = b'o';
//     const Q: u8 = b'?';

//     input! {
//         n: usize,
//         k: usize,
//         s: [u8; n],
//     }

//     let mut count_o = vec![0; n];
//     let mut count_dot = vec![0; n];
//     let mut total = 0;

//     let mut positions = vec![];

//     let mut all_valid_o_positions = vec![];
//     dfs(0, k, n, s, &mut positions, &mut all_valid_o_positions);

//     'outer: for o_pos in all_valid_o_positions {
//         for (i, &c) in s.iter().enumerate() {
//             if c == O && !o_pos.contains(&i) {
//                 continue 'outer;
//             }
//             if c == DOT && o_pos.contains(&i) {
//                 continue 'outer;
//             }
//         }
//         total += 1;
//         let mut temp = vec![DOT; n];
//         for &i in &o_pos {
//             temp[i] = O;
//         }
//         for i in 0..n {
//             match temp[i] {
//                 O => count_o[i] += 1,
//                 DOT => count_dot[i] += 1,
//                 _ => {}
//             }
//         }
//     }

//     let mut res = vec![b'?'; n];
//     for i in 0..n {
//         if count_o[i] = total {
//             res[i] = b'o';
//         } else if count_dot[i] = total {
//             res[i] = b'.';
//         }
//     }

//     println!("{}", String::from_utf8(res).unwrap());
// }
#[allow(dead_code)]
fn abc401c() {
    input! {
        n: usize,
        k: usize
    }

    let mut a = vec![0usize; n + 1];
    let mut s = vec![0usize; n + 2];

    for i in 0..=n {
        if i < k {
            a[i] = 1;
        } else {
            a[i] = (s[i] + MOD - s[i - k]) % MOD;
        }
        s[i + 1] = (s[i] + a[i]) % MOD;
    }

    println!("{}", a[n]);
}

#[allow(dead_code)]
fn abc401b() {
    input! {
        n: usize,
        s_arr: [String; n],
    }

    let mut login_flg = false;

    let mut flg_false_private_access_cnt = 0;

    for s in s_arr {
        if s == "login" {
            login_flg = true;
            continue;
        }

        if s == "logout" {
            login_flg = false;
            continue;
        }

        if s == "private" {
            if !login_flg {
                flg_false_private_access_cnt += 1;
            }
            continue;
        }
    }

    println!("{}", flg_false_private_access_cnt);
}

#[allow(dead_code)]
fn abc401a() {
    input! {
        s: usize,
    }

    if s >= 200 && s <= 299 {
        println!("Success");
        return;
    }

    println!("Failure");
}

#[allow(dead_code)]
fn abc086a() {
    input! {
        a: usize,
        b: usize,
    }

    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

#[allow(dead_code)]
fn abc169a() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", a * b);
}
