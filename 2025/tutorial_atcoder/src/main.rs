use std::{i32, vec};
// Standard library imports
#[allow(unused_imports)]
use std::{
    cmp::{self, Ordering},
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    io::*,
    isize,
    iter::zip,
    str::{self, FromStr},
    usize,
};

// External crate imports
#[allow(unused_imports)]
use itertools::min;
#[allow(unused_imports)]
use proconio::input;

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

#[allow(dead_code)]
const MOD: usize = 1_000_000_000;

fn main() {
    abc105b();
}

#[allow(dead_code)]
fn abc133b() {
}

#[allow(dead_code)]
fn abc105b() {
    input! {
        n: usize,
    }

    for i in 0..=100 {
        for j in 0..=100 {
            if (i * 4) + (j * 7) == n {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

#[allow(dead_code)]
fn abc144b() {
    input! {
        n: usize,
    }

    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == n {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

#[allow(dead_code)]
fn abc075b() {
    input! {
        h: usize,
        w: usize,
        grid: [String; h]
    }

    let mut board: Vec<Vec<char>> = grid.iter().map(|row| row.chars().collect()).collect();

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    for i in 0..h {
        for j in 0..w {
            if board[i][j] == '.' {
                let mut bomb_count = 0;

                for &(di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if ni >= 0 && ni < h as i32 && nj >= 0 && nj < w as i32 {
                        let ui = ni as usize;
                        let uj = nj as usize;
                        if board[ui][uj] == '#' {
                            bomb_count += 1;
                        }
                    }
                }

                board[i][j] = char::from_digit(bomb_count, 10).unwrap();
            }
        }
    }

    for row in board {
        let row_string: String = row.iter().collect();
        println!("{}", row_string);
    }
}


#[allow(dead_code)]
fn abc087b() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut cnt = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if i * 500 + j * 100 + k * 50 == x {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}


#[allow(dead_code)]
fn abc410d_alt() {
    input! {
        n: usize,
        m: usize,
    }

    let mut adj: Vec<Vec<(usize, u16)>> = vec![Vec::new(); n + 1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            w: u16
        }
        adj[a].push((b, w));
    }

    const MAX_X: usize = 1 << 10;
    let mut seen = vec![vec![false; MAX_X]; n + 1];
    let mut que = VecDeque::new();

    seen[1][0] = true;
    que.push_back((1usize, 0usize));

    while let Some((u, x)) = que.pop_front() {
        for &(v, w) in &adj[u] {
            let nx = x ^ w as usize;
            if !seen[v][nx] {
                seen[v][nx] = true;
                que.push_back((v, nx));
            }
        }
    }

    if let Some(ans) = (0..MAX_X).find(|&x| seen[n][x]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
fn abc410c_alt() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a: Vec<usize> = (1..=n).collect();

    let mut shift: usize = 0;
    let mut outs: Vec<usize> = Vec::new();

    for _ in 0..q {
        input! {
            typ: u8
        }

        match typ {
            1 => {
                input! {
                    p: usize,
                    x: usize
                }
                let idx = (p - 1 + shift) % n;
                a[idx] = x;
            }
            2 => {
                input! {
                    p: usize
                }
                let idx = (p - 1 + shift) % n;
                outs.push(a[idx]);
            }
            3 => {
                input! {
                    k: usize
                }
                shift = (shift + k % n) % n;
            }
            _ => unreachable!(),
        }
    }

    if !outs.is_empty() {
        for e in outs {
            println!("{}", e);
        }
    }
}

#[allow(dead_code)]
fn abc410b_alt() {
    input! {
        n: usize,
        q: usize,
        xs: [usize; q]
    }

    let mut cnt = vec![0usize; n];
    let mut ans = Vec::with_capacity(q);

    for x in xs {
        let idx = if x >= 1 {
            x - 1
        } else {
            let mut best = 0;
            for i in 1..n {
                if cnt[i] < cnt[best] {
                    best = i;
                }
            }
            best
        };

        cnt[idx] += 1;
        ans.push(idx + 1);
    }

    for (i, v) in ans.iter().enumerate() {
        if i + 1 == ans.len() {
            println!("{}", v);
        } else {
            print!("{} ", v);
        }
    }
}

#[allow(dead_code)]
fn abc410a_alt() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize,
    }

    let mut cnt = 0;
    for e in a {
        if k <= e {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn abc410d() {
    input! {
        n: usize,
        m: usize,
    }

    let mut adj: Vec<Vec<(usize, u16)>> = vec![Vec::new(); n + 1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            w: u16
        }
        adj[a].push((b, w));
    }

    const MAX_X: usize = 1 << 10;
    let mut seen = vec![vec![false; MAX_X]; n + 1];
    let mut que = VecDeque::new();

    seen[1][0] = true;
    que.push_back((1usize, 0usize));

    while let Some((u, x)) = que.pop_front() {
        for &(v, w) in &adj[u] {
            let nx = x ^ w as usize;
            if !seen[v][nx] {
                seen[v][nx] = true;
                que.push_back((v, nx));
            }
        }
    }

    if let Some(ans) = (0..MAX_X).find(|&x| seen[n][x]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

#[allow(dead_code)]
fn abc410c() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a: Vec<usize> = (1..=n).collect();

    let mut shift: usize = 0;
    let mut outs: Vec<usize> = Vec::new();

    for _ in 0..q {
        input! { typ: u8 }

        match typ {
            1 => {
                input! {
                    p: usize,
                    x: usize
                }
                let idx = (p - 1 + shift) % n;
                a[idx] = x;
            }
            2 => {
                input! { p: usize }
                let idx = (p - 1 + shift) % n;
                outs.push(a[idx]);
            }
            3 => {
                input! { k: usize }
                shift = (shift + k % n) % n;
            }
            _ => unreachable!(),
        }
    }

    if !outs.is_empty() {
        for e in outs {
            println!("{}", e);
        }
    }
}

#[allow(dead_code)]
fn abc410b() {
    input! {
        n: usize,
        q: usize,
        xs: [usize; q]
    }

    let mut cnt = vec![0usize; n];

    let mut ans = Vec::with_capacity(q);

    for x in xs {
        let idx = if x >= 1 {
            x - 1
        } else {
            let mut best = 0;
            for i in 1..n {
                if cnt[i] < cnt[best] {
                    best = i;
                }
            }
            best
        };

        cnt[idx] += 1;
        ans.push(idx + 1);
    }

    for (i, v) in ans.iter().enumerate() {
        if i + 1 == ans.len() {
            println!("{}", v);
        } else {
            print!("{} ", v);
        }
    }
}

#[allow(dead_code)]
fn abc410a() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize,
    }

    let mut cnt = 0;
    for e in a {
        if k <= e {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn abc082b() {
    input! {
        s: String,
        t: String,
    }

    let mut s_dict_ascending: Vec<char> = s.chars().collect();
    let mut t_dict_descending: Vec<char> = t.chars().collect();

    s_dict_ascending.sort();
    t_dict_descending.sort_by(|a, b| b.cmp(a));

    let s_string: String = s_dict_ascending.iter().collect();
    let t_string: String = t_dict_descending.iter().collect();

    // dict comparison
    match s_string.cmp(&t_string) {
        Ordering::Less => println!("Yes"),
        Ordering::Greater => println!("No"),
        Ordering::Equal => println!("No"),
    }
}

#[allow(dead_code)]
fn abc152b() {
    input! {
        a: usize,
        b: usize,
    }

    let mut a_vec = Vec::new();
    let mut b_vec = Vec::new();

    for _ in 0..b {
        a_vec.push(a.to_string());
    }

    for _ in 0..a {
        b_vec.push(b.to_string());
    }

    let a_string = a_vec.join("");
    let b_string = b_vec.join("");

    // dict comparison
    match a_string.cmp(&b_string) {
        Ordering::Less => println!("{}", a_string),
        Ordering::Greater => println!("{}", b_string),
        Ordering::Equal => println!("{}", a_string),
    }
}

#[allow(dead_code)]
fn abc221b() {
    input! {
        s: String,
        t: String,
    }

    if s == t {
        println!("Yes");
        return;
    }

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let n = s_chars.len();

    for i in 0..n - 1 {
        let mut temp = s_chars.clone();
        temp.swap(i, i + 1);

        if temp == t_chars {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

#[allow(dead_code)]
fn abc103b() {
    input! {
        s: String,
        t: String,
    }

    if s.len() != t.len() {
        println!("No");
        return;
    }

    let doubled_t = format!("{}{}", t, t);

    if doubled_t.contains(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn abc066b() {
    input! {
        s: String
    }

    let n = s.len();

    for remove_count in 1..n {
        let remaining_len = n - remove_count;

        if remaining_len % 2 != 0 {
            continue;
        }

        let half = remaining_len / 2;
        let first_half = &s[0..half];
        let second_half = &s[half..remaining_len];

        if first_half == second_half {
            println!("{}", remaining_len);
            return;
        }
    }
}

#[allow(dead_code)]
fn abc058b() {
    input! {
        o: String,
        e: String,
    }

    let mut ans = String::new();

    for i in 0..e.len() {
        ans.push(o.chars().nth(i).unwrap());
        ans.push(e.chars().nth(i).unwrap());
    }

    if o.len() > e.len() {
        ans.push(o.chars().last().unwrap());
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc218b() {
    input! {
        p: [usize; 26],
    }

    let abc = "abcdefghijklmnopqrstuvwxyz";

    let mut ans = String::new();

    for e in p.iter() {
        ans.push(abc.chars().nth(e - 1).unwrap());
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc408d() {
    input! {
        t: usize,
    }

    let mut res: Vec<i32> = Vec::with_capacity(t);

    for _ in 0..t {
        input! {
            n: usize,
            s: String
        }

        let bytes = s.as_bytes();
        debug_assert_eq!(n, bytes.len());

        let mut ones_pref: i32 = 0;
        let mut max_g: i32 = 0;
        let mut min_f: i32 = i32::MAX;
        let mut tot: i32 = 0;

        for (idx, &ch) in bytes.iter().enumerate() {
            if ch == b'1' {
                ones_pref += 1;
                tot += 1;
            }
            let r = (idx as i32) + 1;
            let g_r = r - 2 * ones_pref;

            min_f = min_f.min(g_r - max_g);
            max_g = max_g.max(g_r);
        }

        let ans = tot + min_f.min(0);
        res.push(ans);
    }

    for ans in res {
        println!("{}", ans);
    }
}

#[allow(dead_code)]
fn abc408c_kai() {
    input! {
        n: usize,
        m: usize,
    }

    let mut diff: Vec<i32> = vec![0; n + 2];

    for _ in 0..m {
        input! {
            l: usize,
            r: usize
        }
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let mut cur = 0i32;
    let mut ans = i32::MAX;
    for x in 1..=n {
        cur += diff[x];
        if cur < ans {
            ans = cur;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc408b_kai() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.dedup();

    print!("{} ", a.len());
    for (i, val) in a.iter().enumerate() {
        if i + 1 == a.len() {
            println!("{}", val);
        } else {
            print!("{} ", val);
        }
    }
}

#[allow(dead_code)]
fn abc408a_kai() {
    input! {
        n: usize,
        m: f64,
        mut a: [f64; n]
    }

    let sleep_time = m + 0.5;
    let mut prev_e = 0.0;
    for e in a.iter_mut() {
        if (*e - prev_e).abs() >= sleep_time {
            println!("No");
            return;
        }
        prev_e = *e;
    }

    println!("Yes");
}

#[allow(dead_code)]
// fn abc408d() {
//     input! {
//         t: usize,
//     }

//     let mut res: Vec<i32> = Vec::with_capacity(t);

//     for _ in 0..t {
//         input! {
//             n: usize,
//             s: String,
//         }

//         let bytes = s.as_bytes();
//         debug_assert_eq!(n, bytes.len());

//         let mut ones_pref: i32 = 0;
//         let mut max_g: i32 = 0;
//         let mut min_f: i32 = i32::MAX;
//         let mut tot: i32 = 0;

//         for (idx, &ch) in bytes.iter().enumerate() {
//             if ch == b'1' {
//                 ones_pref += 1;
//                 tot += 1;
//             }
//             let r = (idx as i32) + 1;
//             let g_r = r - 2 * ones_pref;

//             min_f = min_f.min(g_r - max_g);
//             max_g = max_g.max(g_r);
//         }

//         let ans = tot + min_f.min(0);
//         res.push(ans);
//     }

//     for ans in res {
//         println!("{}", ans);
//     }
// }
#[allow(dead_code)]
fn abc408c() {
    input! {
        n: usize,
        m: usize,
    }

    let mut diff: Vec<i32> = vec![0; n + 2];

    for _ in 0..m {
        input! {
            l: usize,
            r: usize
        }
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let mut cur = 0i32;
    let mut ans = i32::MAX;
    for x in 1..=n {
        cur += diff[x];
        if cur < ans {
            ans = cur;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc408b() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.dedup();

    print!("{} ", a.len());
    for (i, val) in a.iter().enumerate() {
        if i + 1 == a.len() {
            println!("{}", val);
        } else {
            print!("{} ", val);
        }
    }
}

#[allow(dead_code)]
fn abc408a() {
    input! {
        n: usize,
        m: f64,
        mut a: [f64; n]
    }

    let sleep_time = m + 0.5;
    let mut prev_e = 0.0;
    for e in a.iter_mut() {
        if (*e - prev_e).abs() >= sleep_time {
            println!("No");
            return;
        }
        prev_e = *e;
    }

    println!("Yes");
}

// =============================================================================
// ABC 147 Problems
// =============================================================================

#[allow(dead_code)]
fn abc147b() {
    input! {
        s: String
    }

    let palindrome = s.chars().rev().collect::<String>();
    let mut cnt = 0;

    for (c1, c2) in s.chars().zip(palindrome.chars()) {
        if c1 != c2 {
            cnt += 1;
        }
    }

    println!("{}", cnt / 2);
}

// =============================================================================
// ABC 146 Problems
// =============================================================================

#[allow(dead_code)]
fn abc146b() {
    input! {
        n: usize,
        s: String,
    }

    let mut ans_str = String::new();
    let abc = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for c in s.chars() {
        let mut idx = abc.find(c).unwrap();
        idx = (idx + n) % 26;
        ans_str.push(abc.chars().nth(idx).unwrap());
    }

    println!("{}", ans_str);
}

// =============================================================================
// ABC 215 Problems
// =============================================================================

#[allow(dead_code)]
fn abc215a() {
    input! {
        s: String,
    }

    if s == "Hello,World!" {
        println!("AC");
    } else {
        println!("WA");
    }
}

// =============================================================================
// ABC 084 Problems
// =============================================================================

#[allow(dead_code)]
fn abc084b() {
    input! {
        a: usize,
        b: usize,
        s: String
    }

    if !s.contains('-') {
        println!("No");
        return;
    }

    let mut parts = s.split('-');
    let first_part = parts.next().unwrap();
    let second_part = parts.next().unwrap();

    if first_part.len() == a && second_part.len() == b {
        if first_part.chars().all(|c| c.is_digit(10)) && second_part.chars().all(|c| c.is_digit(10))
        {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}

// =============================================================================
// ABC 407 Problems
// =============================================================================

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

// =============================================================================
// ABC 104 Problems
// =============================================================================

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

// =============================================================================
// Special Problems (Tenkei)
// =============================================================================

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

    // Sliding window approach
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
}

// =============================================================================
// Utility Functions
// =============================================================================

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

// =============================================================================
// ABC 113 Problems
// =============================================================================

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

// =============================================================================
// ABC 113 Problems (continued)
// =============================================================================

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

// =============================================================================
// ABC 121 Problems
// =============================================================================

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

// =============================================================================
// ABC 115 Problems
// =============================================================================

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

// =============================================================================
// ABC 132 Problems
// =============================================================================

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

// =============================================================================
// ABC 067 Problems
// =============================================================================

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

// =============================================================================
// ABC 205 Problems
// =============================================================================

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

// =============================================================================
// ABC 404 Problems
// =============================================================================

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
}

// =============================================================================
// ABC 201 Problems
// =============================================================================

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

// =============================================================================
// ABC 124 Problems
// =============================================================================

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

// =============================================================================
// ABC 095 Problems
// =============================================================================

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

// =============================================================================
// ABC 130 Problems
// =============================================================================

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

// =============================================================================
// ABC 174 Problems
// =============================================================================

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

// =============================================================================
// ABC 088 Problems
// =============================================================================

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

// =============================================================================
// ABC 081 Problems
// =============================================================================

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

// =============================================================================
// ABC 220 Problems
// =============================================================================

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

// =============================================================================
// ABC 090 Problems
// =============================================================================

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

// =============================================================================
// ABC 068 Problems
// =============================================================================

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

// =============================================================================
// ABC 158 Problems
// =============================================================================

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

// =============================================================================
// ABC 093 Problems
// =============================================================================

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

// =============================================================================
// ABC 208 Problems
// =============================================================================

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

// =============================================================================
// ABC 164 Problems
// =============================================================================

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

// =============================================================================
// ABC 200 Problems
// =============================================================================

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

// =============================================================================
// ABC 165 Problems
// =============================================================================

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

// =============================================================================
// ABC 206 Problems
// =============================================================================

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

// =============================================================================
// ABC 162 Problems
// =============================================================================

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

// =============================================================================
// ABC 153 Problems
// =============================================================================

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

// =============================================================================
// Helper/Utility Functions
// =============================================================================

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

// =============================================================================
// ABC 401 Problems
// =============================================================================

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
