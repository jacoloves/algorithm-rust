use std::cmp;
use std::cmp::min;
use std::fmt::Debug;
use std::ptr::read;
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
    abc132_c();
}

fn abc132_c() {
    let _l: Vec<usize> = read_line();
    let mut v: Vec<usize> = read_line();

    v.sort();

    let ans = v[v.len() / 2] - v[v.len() / 2 - 1];

    println!("{}", ans);
}

fn abc161_c() {
    let l: Vec<i128> = read_line();
    let n = l[0];
    let k = l[1];

    let a = n % k;
    println!("{}", min(a, k - a));
}

fn agc014_a() {
    let l: Vec<usize> = read_line();
    let mut a = l[0];
    let mut b = l[1];
    let mut c = l[2];

    let mut cnt = 0;

    if a % 2 == 1 || b % 2 == 1 || c % 2 == 1 {
        println!("0");
    } else if a == b || a == c {
        println!("-1");
    } else {
        while a % 2 == 0 && b % 2 == 0 && c % 2 == 0 {
            let a_c = a.clone();
            let b_c = b.clone();
            let c_c = c.clone();
            a = (b_c + c_c) / 2;
            b = (a_c + c_c) / 2;
            c = (a_c + b_c) / 2;
            cnt += 1;
        }

        println!("{}", cnt);
    }
}

fn abc160_c() {
    let l: Vec<usize> = read_line();
    let k = l[0];
    let n = l[1];
    let mut a: Vec<usize> = read_line();

    a.push(k + a[0]);

    let mut max_range: usize = 0;
    for i in 0..n {
        max_range = cmp::max(max_range, a[i + 1] - a[i]);
    }

    println!("{}", k - max_range);
}

fn abc068_b() {
    let l: Vec<i32> = read_line();
    let n = l[0];

    if n == 1 {
        println!("1");
    } else {
        let mut max_cnt = -1;

        let mut ans = 0;
        for i in 2..=n {
            let mut div_i = i.clone();
            let mut cnt = 0;
            while div_i / 2 != 0 {
                div_i = div_i / 2;
                cnt += 1;
                if max_cnt < cnt {
                    max_cnt = cnt;
                    ans = i;
                }
            }
        }

        println!("{}", ans);
    }
}

fn abc088_b() {
    let l: Vec<i32> = read_line();
    let n = l[0];
    let mut a: Vec<i32> = read_line();

    a.sort();
    a.reverse();

    let mut ans = 0;
    for (a_e, i) in a.iter().zip(1..=n) {
        if i % 2 != 0 {
            ans += a_e;
        } else {
            ans -= a_e;
        }
    }

    println!("{}", ans);
}

fn abc074_b() {
    let l: Vec<i32> = read_line();
    let _n = l[0];
    let l: Vec<i32> = read_line();
    let k = l[0];
    let x: Vec<i32> = read_line();

    let mut ans = 0;
    for x_e in x.iter() {
        ans += min(x_e * 2, (k - x_e) * 2);
    }

    println!("{}", ans);
}

fn abc086_b() {
    let l: Vec<String> = read_line();

    let syn_str = l[0].clone() + &l[1].clone();

    let num: i32 = syn_str.parse().unwrap();

    let mut ans = "No";
    for i in 1..=2000 {
        if num == i * i {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}

fn abc175_b() {
    let b_1: Vec<usize> = read_line();
    let b_2: Vec<usize> = read_line();
    let b_3: Vec<usize> = read_line();

    let mut bc_1 = [0, 0, 0];
    let mut bc_2 = [0, 0, 0];
    let mut bc_3 = [0, 0, 0];

    let l: Vec<usize> = read_line();
    let n = l[0];

    for _ in 1..=n {
        let b: Vec<usize> = read_line();
        for (j, e) in (0..).zip(b_1.iter()) {
            if *e == b[0] {
                bc_1[j] = 1;
            }
        }
        for (j, e) in (0..).zip(b_2.iter()) {
            if *e == b[0] {
                bc_2[j] = 1;
            }
        }
        for (j, e) in (0..).zip(b_3.iter()) {
            if *e == b[0] {
                bc_3[j] = 1;
            }
        }
    }

    if (bc_1[0] == 1 && bc_1[1] == 1 && bc_1[2] == 1)
        || (bc_2[0] == 1 && bc_2[1] == 1 && bc_2[2] == 1)
        || (bc_3[0] == 1 && bc_3[1] == 1 && bc_3[2] == 1)
        || (bc_1[0] == 1 && bc_2[0] == 1 && bc_3[0] == 1)
        || (bc_1[1] == 1 && bc_2[1] == 1 && bc_3[1] == 1)
        || (bc_1[2] == 1 && bc_2[2] == 1 && bc_3[2] == 1)
        || (bc_1[0] == 1 && bc_2[1] == 1 && bc_3[2] == 1)
        || (bc_1[2] == 1 && bc_2[1] == 1 && bc_3[0] == 1)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn panasonic_2020_b() {
    let l: Vec<i128> = read_line();
    let h = l[0];
    let w = l[1];

    if h == 1 || w == 1 {
        println!("1");
    } else {
        let mut ans = h * w / 2;
        if (h * w) % 2 == 1 {
            ans += 1;
        }

        println!("{}", ans);
    }
}

fn abc121_b() {
    let l: Vec<i32> = read_line();
    let n = l[0];
    let c = l[2];

    let bl: Vec<i32> = read_line();

    let mut cnt = 0;
    for _ in 1..=n {
        let al: Vec<i32> = read_line();
        let mut sum = 0;
        for (a, b) in al.iter().zip(bl.iter()) {
            sum += a * b;
        }

        if sum + c > 0 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

fn submitb_2019_b() {
    let l: Vec<f32> = read_line();

    let n: f32 = l[0];

    let m = n / 1.08;
    let c1 = m.floor();
    let c2 = c1 - 1.0;
    let c3 = c1 + 1.0;
    if (c1 * 1.08).floor() == n {
        println!("{}", c1 as i32);
    } else if (c2 * 1.08).floor() == n {
        println!("{}", c2 as i32);
    } else if (c3 * 1.08).floor() == n {
        println!("{}", c3 as i32);
    } else {
        println!(":(");
    }
}

fn code_fes_qual_b() {
    let l: Vec<usize> = read_line();

    let _n: usize = l[0];
    let a: usize = l[1];
    let b: usize = l[2];

    let s_vec: Vec<String> = read_line();

    let s: String = s_vec[0].clone();

    let pass_limit: usize = a + b;

    let mut foreign_cnt: usize = 1;

    let mut pass_cnt: usize = 1;
    for c in s.chars() {
        if c == 'a' {
            if pass_limit >= pass_cnt {
                println!("Yes");
                pass_cnt += 1;
            } else {
                println!("No");
            }
        } else if c == 'b' {
            if pass_limit >= pass_cnt && b >= foreign_cnt {
                println!("Yes");
                pass_cnt += 1;
                foreign_cnt += 1;
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    }
}

fn double_points() {
    let s: Vec<String> = read_line();

    let str1: &str = &s[0];

    let chang_str1 = str1
        .chars()
        .map(|c| format!("{}{}", c, "``"))
        .collect::<String>();

    println!("{}", chang_str1);
}

fn abc156c() {
    let l: Vec<usize> = read_line();
    let N: usize = l[0];

    let x: Vec<isize> = read_line();

    let mut ans: isize = 1 << 30;
    for p in 1..=100 {
        let mut cnt: isize = 0;
        for i in 0..N {
            cnt += (x[i] - p).pow(2);
        }
        ans = min(ans, cnt);
    }

    println!("{}", ans);
}

fn abc193_ans() {
    let l: Vec<u32> = read_line();

    let a: u32 = l[0];
    let b: u32 = l[1];

    let ans: u32 = (b - 1 + a - 2) / (a - 1);

    println!("{}", ans);
}

fn abc193() {
    let l: Vec<u32> = read_line();

    let a: u32 = l[0];
    let b: u32 = l[1];

    let mut ans: u32;

    let mut plug: u32 = 0;
    let mut cnt: u32 = 0;
    let max_num: u32 = 1000000007;
    for i in 1..=max_num {
        cnt += 1;
        ans = a * i - plug;
        if ans >= b {
            break;
        }
        plug += 1;
    }

    println!("{}", cnt);
}
