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

fn main() {
    itp1_8_b();
}

fn itp1_7_a() {
    let mut v: Vec<String> = Vec::new();
    loop {
        let l: Vec<i32> = read_line();
        if l[0] == -1 && l[1] == -1 && l[2] == -1 {
            break;
        }

        let mut m: i32 = l[0];
        let mut f: i32 = l[1];

        if m == -1 || f == -1 {
            v.push("F".to_string());
            continue;
        }

        let jud1: i32 = m + f;
        if jud1 >= 80 {
            v.push("A".to_string());
        } else if jud1 >= 65 && jud1 < 80 {
            v.push("B".to_string());
        } else if jud1 >= 50 && jud1 < 65 {
            v.push("C".to_string());
        } else if jud1 >= 30 && jud1 < 50 {
            if l[2] >= 50 {
                v.push("C".to_string());
            } else {
                v.push("D".to_string());
            }
        } else {
            v.push("F".to_string());
        }
    }

    for e in v {
        println!("{}", e);
    }
}

fn itp1_7_b() {
    loop {
        let mut ans: i32 = 0;
        let l: Vec<i32> = read_line();
        if l[0] == 0 && l[1] == 0 {
            break;
        }
        for i in 1..=l[0] - 2 {
            for j in i + 1..=l[0] - 1 {
                for k in j + 1..=l[0] {
                    if l[1] == i + j + k {
                        ans = ans + 1;
                    }
                }
            }
        }
        println!("{}", ans);
    }
}

fn itp1_7_c() {
    let l: Vec<i32> = read_line();
    let first: i32 = l[0];
    let second: i32 = l[1];

    let mut v: Vec<Vec<usize>> = vec![vec![0; second as usize]; first as usize];

    for i in 0..first {
        let inp: Vec<usize> = read_line();
        for j in 0..second {
            v[i as usize][j as usize] = inp[j as usize];
        }
    }

    let mut ansv: Vec<Vec<usize>> = vec![vec![0; (second + 1) as usize]; (first + 1) as usize];

    for i in 0..first {
        let mut sumw: usize = 0;

        for j in 0..second {
            ansv[i as usize][j as usize] = v[i as usize][j as usize];
            sumw = sumw + v[i as usize][j as usize];
        }
        ansv[i as usize][second as usize] = sumw;
    }

    let mut lasth = 0;
    for i in 0..second {
        let mut sumh: usize = 0;
        for j in 0..first {
            sumh = sumh + v[j as usize][i as usize];
        }
        ansv[first as usize][i as usize] = sumh;
        lasth = lasth + sumh;
    }

    ansv[first as usize][second as usize] = lasth;

    for ev in ansv.iter() {
        let mut cnt: i32 = 1;
        for e in ev.iter() {
            if cnt == second + 1 {
                println!("{}", e);
            } else {
                print!("{} ", e);
            }
            cnt = cnt + 1;
        }
    }
}

fn itp1_7_d() {
    let line: Vec<usize> = read_line();

    let n: usize = line[0];
    let m: usize = line[1];
    let l: usize = line[2];

    let mut av: Vec<Vec<usize>> = vec![vec![0; m]; n];
    let mut bv: Vec<Vec<usize>> = vec![vec![0; l]; m];
    let mut cv: Vec<Vec<usize>> = vec![vec![0; l]; n];

    for i in 0..n {
        let line: Vec<usize> = read_line();
        for j in 0..m {
            av[i][j] = line[j];
        }
    }

    for i in 0..m {
        let line2: Vec<usize> = read_line();
        for j in 0..l {
            bv[i][j] = line2[j];
        }
    }

    for i in 0..n {
        for j in 0..l {
            let mut calc: usize = 0;
            for k in 0..m {
                calc = calc + (av[i][k] * bv[k][j]);
            }
            cv[i][j] = calc;
        }
    }

    for i in 0..n {
        for j in 0..l {
            if j == l - 1 {
                println!("{}", cv[i][j]);
            } else {
                print!("{} ", cv[i][j]);
            }
        }
    }
}

fn itp1_8_a() {
    let l: Vec<String> = read_line();

    let mut i: usize = 1;
    for ev in l.iter() {
        for c in ev.chars() {
            if c.is_lowercase() {
                print!("{}", c.to_uppercase());
            } else {
                print!("{}", c.to_lowercase());
            }
        }
        if i == l.iter().count() {
            println!();
        } else {
            print!(" ");
        }
        i = i + 1;
    }
}

fn itp1_8_b() {
    let mut v: Vec<u16> = Vec::new();
    loop {
        let l: Vec<String> = read_line();
        let x: String = l[0].clone();
        let mut ans: u16 = 0;

        if x == "0" {
            break;
        }

        for c in x.chars() {
            ans += c as u16 - '0' as u16;
        }

        v.push(ans);
    }

    for e in v.iter() {
        println!("{}", e);
    }
}
