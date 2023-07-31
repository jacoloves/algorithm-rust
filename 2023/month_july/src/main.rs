extern crate itertools;
extern crate proconio;

#[allow(unused_imports)]
use itertools::Itertools;

#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use std::{isize, usize};

/* ↓AOJ */
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

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
/* ↑AOJ */

fn main() {
    alds1_4_b();
}

fn binary_search(a: &Vec<i32>, key: i32) -> bool {
    let mut left = 0;
    let mut right = a.len();

    while left < right {
        let mid = (left + right) / 2;
        if a[mid] == key {
            return true;
        } else if key < a[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    return false;
}

#[allow(dead_code)]
fn alds1_4_b() {
    let l: Vec<String> = read_line();
    let _ = l[0].parse::<usize>().unwrap();

    let l: Vec<String> = read_line();
    let s = l.clone();

    let i_s: Vec<i32> = s.iter().map(|e| e.parse::<i32>().unwrap()).collect();

    let l: Vec<String> = read_line();
    let _ = l[0].parse::<usize>().unwrap();

    let l: Vec<String> = read_line();
    let t = l.clone();

    let i_t: Vec<i32> = t.iter().map(|e| e.parse::<i32>().unwrap()).collect();

    let mut cnt = 0;

    for e in i_t.iter() {
        if binary_search(&i_s, *e) {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn alds1_4_a() {
    let l: Vec<String> = read_line();
    let _ = l[0].parse::<usize>().unwrap();

    let l: Vec<String> = read_line();
    let s = l.clone();

    let l: Vec<String> = read_line();
    let _ = l[0].parse::<usize>().unwrap();

    let mut cnt = 0;

    let l: Vec<String> = read_line();
    let t = l.clone();

    for e in t.iter() {
        if s.contains(&e) {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn alds1_3_d() {
    let l: Vec<String> = read_line();
    let s = l[0].clone();

    let n = s.len();

    let mut st: VecDeque<(char, i32)> = VecDeque::new();
    let mut v: Vec<(i32, i32)> = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '\\' {
            st.push_back((c, i as i32));
        } else if c == '/' {
            if st.len() != 0 && st.back().unwrap().0 == '\\' {
                let (c, j) = st.pop_back().unwrap();
                v.push((j, i as i32));
            } else {
                st.push_back((c, i as i32));
            }
        }
    }

    v.sort_by(|a, b| a.0.cmp(&b.0));

    let mut a = -1;
    let mut b = 0;
    let mut sum = 0;
    let mut ans: Vec<i32> = Vec::new();

    for e in v.iter() {
        if e.0 > a {
            if b > 0 {
                ans.push(b);
            }
            b = 0;
            a = e.1;
        }
        sum += e.1 - e.0;
        b += e.1 - e.0;
    }

    if b > 0 {
        ans.push(b);
    }

    println!("{}", sum);
    print!("{}", ans.len());
    for e in ans.iter() {
        print!(" {}", e);
    }
    println!("");
}

#[allow(dead_code)]
fn alds1_3_c() {
    let l: Vec<String> = read_line();
    let n = l[0].parse::<usize>().unwrap();

    let mut list: VecDeque<i32> = VecDeque::new();

    for _ in 0..n {
        let l: Vec<String> = read_line();
        let command = l[0].clone();
        if command == "insert" {
            let a = l[1].clone().parse::<i32>().unwrap();
            list.push_front(a);
        } else if command == "delete" {
            let a = l[1].clone().parse::<i32>().unwrap();
            for (i, e) in list.iter().enumerate() {
                if *e == a {
                    let idx = i;
                    list.remove(idx);
                    break;
                }
            }
        } else if command == "deleteFirst" {
            list.pop_front();
        } else if command == "deleteLast" {
            list.pop_back();
        }
    }

    for (e, i) in list.iter().zip(0..list.len()) {
        if i != list.len() - 1 {
            print!("{} ", e);
        } else {
            println!("{}", e);
        }
    }
}

#[allow(dead_code)]
fn abc311_c() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut fl = vec![0; n];
    let mut s = Vec::new();
    let mut v: i32 = 1;

    while fl[v as usize - 1] == 0 {
        fl[v as usize - 1] = 1;
        s.push(v);
        v = a[v as usize - 1] as i32;
    }

    let mut res = Vec::new();
    for &nx in &s {
        if nx == v {
            v = -1;
        }
        if v == -1 {
            res.push(nx);
        }
    }

    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
}

#[allow(dead_code)]
fn abc311_b() {
    input! {
        n: usize,
        d: usize,
        s: [String; n],
    }

    let mut days: Vec<Vec<i32>> = Vec::new();

    for e in s.iter() {
        let mut arr: Vec<i32> = Vec::new();
        for (c, i) in e.chars().zip(0..d) {
            if c == 'o' {
                arr.push(i as i32 + 1);
            }
        }
        days.push(arr);
    }

    let mut ele: Option<HashSet<_>> = None;

    for day in days {
        let set: HashSet<_> = day.into_iter().collect();
        ele = match ele {
            Some(com) => Some(com.intersection(&set).cloned().collect()),
            None => Some(set),
        };
    }

    if let Some(com) = ele {
        let mut com_v: Vec<_> = com.into_iter().collect();
        com_v.sort();
        let mut max = 0;
        let mut tmp = 0;
        let mut cnt = 0;
        for e in com_v {
            if cnt != 0 {
                if e == tmp + 1 {
                    cnt += 1;
                } else {
                    cnt = 1;
                }
            } else {
                cnt += 1;
            }
            max = cmp::max(max, cnt);
            tmp = e;
        }
        println!("{}", max);
    } else {
        println!("0");
    }
}

#[allow(dead_code)]
fn abc311_a() {
    input! {
        _: usize,
        s: String,
    }

    let mut a_cnt = 0;
    let mut b_cnt = 0;
    let mut c_cnt = 0;

    for c in s.chars() {
        if c == 'A' {
            a_cnt += 1;
        } else if c == 'B' {
            b_cnt += 1;
        } else {
            c_cnt += 1;
        }

        if a_cnt >= 1 && b_cnt >= 1 && c_cnt >= 1 {
            break;
        }
    }

    println!("{}", a_cnt + b_cnt + c_cnt);
}

#[allow(dead_code)]
fn alds1_3_b() {
    let l: Vec<i32> = read_line();
    let n = l[0];
    let q = l[1];

    let mut que1: VecDeque<(String, i32)> = VecDeque::new();
    let mut ans: VecDeque<(String, i32)> = VecDeque::new();

    for _ in 0..n {
        let l: Vec<String> = read_line();
        let name = l[0].clone();
        let time = l[1].clone().parse::<i32>().unwrap();

        que1.push_back((name, time));
    }

    let mut cnt = 0;
    while ans.len() != n as usize {
        if que1[0].1 > q {
            que1[0].1 -= q;
            cnt += q;
            // first que change
            let tmp = que1.pop_front().unwrap();
            que1.push_back(tmp);
        } else {
            cnt += que1[0].1;
            // ans push_back
            let mut ans_tmp = que1.pop_front().unwrap();
            ans_tmp.1 = cnt;
            ans.push_back(ans_tmp);
        }
    }

    for (n, t) in ans.iter() {
        println!("{} {}", n, t);
    }
}

#[allow(dead_code)]
fn alds1_3_a() {
    let l: Vec<String> = read_line();
    let str_pol = l;

    let mut v: Vec<i64> = vec![];

    for elem in str_pol.iter() {
        if let Ok(num) = elem.parse::<i64>() {
            v.push(num);
        } else {
            if elem == "+" {
                let pop2 = v.pop().unwrap();
                let pop1 = v.pop().unwrap();
                let res = pop1 + pop2;
                v.push(res);
            } else if elem == "-" {
                let pop2 = v.pop().unwrap();
                let pop1 = v.pop().unwrap();
                let res = pop1 - pop2;
                v.push(res);
            } else {
                let pop2 = v.pop().unwrap();
                let pop1 = v.pop().unwrap();
                let res = pop1 * pop2;
                v.push(res);
            }
        }
    }

    let ans = v.pop().unwrap();

    println!("{}", ans);
}

fn insertion_sort(v: &mut Vec<i32>, n: usize, g: usize) -> i32 {
    let mut cnt = 0;
    for i in g..n {
        let ve = v[i];
        let mut j = i;
        while j >= g && v[j as usize - g] > ve {
            v[j] = v[j - g];
            j -= g;
            cnt += 1;
        }
        v[j] = ve;
    }
    cnt
}

fn shell_sort(v: &mut Vec<i32>, n: usize) {
    let mut cnt = 0;
    let mut m = 0;
    let mut g = vec![];

    let mut i = 1;
    while i <= n {
        g.push(i);
        i = i * 3 + 1;
        m += 1;
    }

    g.reverse();

    for i in 0..m {
        cnt += insertion_sort(v, n, g[i]);
    }

    println!("{}", m);
    for i in 0..m {
        if i != m - 1 {
            print!("{} ", g[i])
        } else {
            println!("{}", g[i]);
        }
    }

    println!("{}", cnt);
    for i in 0..n {
        if i != n - 1 {
            print!("{} ", v[i])
        } else {
            println!("{}", v[i]);
        }
    }
}

#[allow(dead_code)]
fn alds1_2_d() {
    let l: Vec<usize> = read_line();
    let n = l[0];
    let mut a = vec![];

    for i in 0..n {
        let l: Vec<i32> = read_line();
        a.push(l[0]);
    }

    shell_sort(&mut a, n);
}

#[allow(dead_code)]
fn alds1_2_c() {
    let l: Vec<usize> = read_line();
    let n = l[0];

    let mut bubble_sort: Vec<(char, i32)> = Vec::new();
    //    let mut select_sort: Vec<(char, i32)> = Vec::new();

    let l: Vec<String> = read_line();
    for i in 0..n {
        let chars: Vec<char> = l[i].chars().collect();
        let letter = chars[0];
        let number = chars[1..]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        bubble_sort.push((letter, number));
    }

    let mut selection_sort = bubble_sort.clone();

    // bubblesort
    for i in 0..n {
        for j in (i + 1..n).rev() {
            if bubble_sort[j].1 < bubble_sort[j - 1].1 {
                bubble_sort.swap(j, j - 1);
            }
        }
    }

    let bubble_string: Vec<String> = bubble_sort
        .iter()
        .map(|a| a.0.to_string() + &a.1.to_string())
        .collect();

    println!("{}", bubble_string.join(" "));
    println!("Stable");

    // selectionsort
    for i in 0..n {
        let mut minj = i;
        for j in i..n {
            if selection_sort[j].1 < selection_sort[minj].1 {
                minj = j;
            }
        }
        if i != minj {
            selection_sort.swap(i, minj);
        }
    }

    let selection_string: Vec<String> = selection_sort
        .iter()
        .map(|a| a.0.to_string() + &a.1.to_string())
        .collect();

    println!("{}", selection_string.join(" "));

    if bubble_string.join(" ").eq(&selection_string.join(" ")) {
        println!("Stable");
    } else {
        println!("Not stable");
    }
}

#[allow(dead_code)]
fn alds1_2_b() {
    let l: Vec<usize> = read_line();
    let n = l[0];

    let mut a = vec![];

    let l: Vec<i32> = read_line();
    for i in 0..n {
        a.push(l[i]);
    }

    let mut cnt = 0;
    for i in 0..n {
        let mut minj = i;
        for j in i..n {
            if a[j] < a[minj] {
                minj = j;
            }
        }
        if a[i] != a[minj] {
            let tmp = a[i];
            a[i] = a[minj];
            a[minj] = tmp;
            cnt += 1;
        }
    }

    for i in 0..n {
        if i == n - 1 {
            println!("{}", a[i]);
        } else {
            print!("{} ", a[i]);
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn alds1_2_a() {
    let l: Vec<usize> = read_line();
    let n = l[0];

    let mut r = vec![];

    let l: Vec<i32> = read_line();
    for i in 0..n {
        r.push(l[i]);
    }

    let mut flag = true;
    let mut cnt = 0;
    while flag {
        flag = false;
        for j in (1..n).rev() {
            if r[j] < r[j - 1] {
                let tmp = r[j];
                r[j] = r[j - 1];
                r[j - 1] = tmp;
                flag = true;
                cnt += 1;
            }
        }
    }

    for i in 0..n {
        if i == n - 1 {
            println!("{}", r[i]);
        } else {
            print!("{} ", r[i]);
        }
    }

    println!("{}", cnt);
}

#[allow(dead_code)]
fn alds1_1_d() {
    let l: Vec<usize> = read_line();
    let n = l[0];

    let mut r = vec![];

    for _ in 1..=n {
        let l: Vec<i32> = read_line();
        r.push(l[0]);
    }

    let mut minv: i32 = r[0];
    let mut maxv: i32 = -1000000009;
    for j in 1..n {
        maxv = cmp::max(maxv, r[j] - minv);
        minv = cmp::min(minv, r[j]);
    }

    println!("{}", maxv);
}

fn is_prime(x: usize) -> bool {
    if x == 2 {
        return true;
    }

    if x < 2 || x % 2 == 0 {
        return false;
    }

    //let mut i: usize = 3;
    let limit = (x as f64).sqrt() as usize;

    for i in 3..=limit {
        if x % i == 0 {
            return false;
        }
    }

    return true;
}

#[allow(dead_code)]
fn alds1_1_c() {
    let l: Vec<usize> = read_line();
    let n = l[0];

    let mut v = vec![];

    for _ in 1..=n {
        let l: Vec<usize> = read_line();
        v.push(l[0]);
    }

    let mut cnt: usize = 0;
    for i in 0..n {
        if is_prime(v[i]) {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

#[allow(dead_code)]
fn alds1_1_b() {
    let l: Vec<usize> = read_line();
    let mut x = l[0];
    let mut y = l[1];

    if x >= y {
        println!("{}", gcd(x, y));
    } else {
        println!("{}", gcd(y, x));
    }
}

#[allow(dead_code)]
fn alds1_1_a() {
    let l: Vec<usize> = read_line();
    let n = l[0];

    let mut a: Vec<usize> = read_line();

    for (e, i) in a.iter().zip(0..n) {
        if i == n - 1 {
            println!("{}", e);
        } else {
            print!("{} ", e);
        }
    }

    for i in 1..n {
        let v = a[i];
        let mut j: i32 = i as i32 - 1;
        while j >= 0 && a[j as usize] > v {
            a[j as usize + 1] = a[j as usize];
            j -= 1;
        }
        let k: usize = (j + 1) as usize;
        a[k] = v;
        for (e, i) in a.iter().zip(0..n) {
            if i == n - 1 {
                println!("{}", e);
            } else {
                print!("{} ", e);
            }
        }
    }
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
