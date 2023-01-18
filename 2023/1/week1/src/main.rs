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
    itp1_6_a();
}

fn itp1_6_a() {
    let _: Vec<i32> = read_line();
    let mut line: Vec<i64> = read_line();

    line.reverse();

    for i in 0..line.len() {
        if i == line.len() - 1 {
            print!("{}", line[i]);
        } else {
            print!("{} ", line[i]);
        }
    }

    println!();
}

fn itp1_5_d() {
    let line: Vec<i32> = read_line();
    let mut ans: Vec<String> = Vec::new();

    let mut i: i32 = 0;
    loop {
        i = i + 1;
        // END_CHECK_NUM
        if i > line[0] {
            break;
        }
        // CHECK_NUM
        let mut x: i32 = i.clone();
        if x % 3 == 0 {
            ans.push(format!(" {}", i));
            continue;
        }
        // INCLUDE3
        loop {
            if x % 10 == 3 {
                ans.push(format!(" {}", i));
                break;
            }

            if x == 0 {
                break;
            }

            x = x / 10;
        }
    }

    for e in ans.iter() {
        print!("{}", e);
    }
    println!();
}

fn itp1_5_c() {
    let mut wv: Vec<i32> = Vec::new();
    let mut hv: Vec<i32> = Vec::new();
    loop {
        let line: Vec<i32> = read_line();
        if line[0] == 0 && line[1] == 0 {
            break;
        }

        hv.push(line[0]);
        wv.push(line[1]);
    }

    let mut reverse: bool;
    for i in 0..hv.len() {
        for j in 1..=hv[i] {
            if j % 2 != 0 {
                reverse = true;
            } else {
                reverse = false;
            }
            for k in 1..=wv[i] {
                if reverse {
                    if k % 2 != 0 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                } else {
                    if k % 2 != 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
            }
            println!();
        }
        println!();
    }
}

fn itp1_5_b() {
    let mut wv: Vec<i32> = Vec::new();
    let mut hv: Vec<i32> = Vec::new();
    loop {
        let line: Vec<i32> = read_line();
        if line[0] == 0 && line[1] == 0 {
            break;
        }

        hv.push(line[0]);
        wv.push(line[1]);
    }

    for i in 0..hv.len() {
        for j in 1..=hv[i] {
            for k in 1..=wv[i] {
                if j == 1 || j == hv[i] {
                    print!("#");
                } else {
                    if k == 1 || k == wv[i] {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!();
    }
}

fn itp1_5_a() {
    let mut wv: Vec<i32> = Vec::new();
    let mut hv: Vec<i32> = Vec::new();
    loop {
        let line: Vec<i32> = read_line();
        if line[0] == 0 && line[1] == 0 {
            break;
        }

        hv.push(line[0]);
        wv.push(line[1]);
    }

    for i in 0..hv.len() {
        for _ in 1..=hv[i] {
            for _ in 1..=wv[i] {
                print!("#");
            }
            println!();
        }
        println!();
    }
}

fn itp1_4_d() {
    let _: Vec<i32> = read_line();
    let line: Vec<i64> = read_line();

    let max_num: i64 = *line.iter().max().unwrap();
    let min_num: i64 = *line.iter().min().unwrap();
    let sum_num: i64 = line.iter().sum();

    println!("{} {} {}", min_num, max_num, sum_num);
}

fn itp1_4_c() {
    let mut ansv: Vec<i32> = Vec::new();
    loop {
        let line: Vec<String> = read_line();
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut op: String = "".to_string();
        let mut ans: i32 = 0;

        let mut cnt: i32 = 1;
        for e in line.iter() {
            match cnt {
                1 => {
                    a = e.parse().unwrap();
                }
                2 => {
                    op = e.to_string();
                }
                _ => {
                    b = e.parse().unwrap();
                }
            }
            cnt = cnt + 1;
        }

        match op.as_str() {
            "+" => {
                ans = a + b;
            }
            "-" => {
                ans = a - b;
            }
            "*" => {
                ans = a * b;
            }
            "/" => {
                ans = a / b;
            }
            _ => {
                break;
            }
        }

        ansv.push(ans);
    }

    for e in ansv.iter() {
        println!("{}", e);
    }
}

fn itp1_4_b() {
    let line: Vec<f64> = read_line();

    let area: f64 = line[0].powf(2.0) * std::f64::consts::PI;
    let circle: f64 = 2.0 * std::f64::consts::PI * line[0];

    println!("{} {}", area, circle);
}

fn itp1_4_a() {
    let line: Vec<i32> = read_line();

    let a: i32 = line[0];
    let b: i32 = line[1];

    let d: i32 = a / b;
    let r: i32 = a % b;
    let f: f64 = a as f64 / b as f64;

    println!("{} {} {}", d, r, f);
}
