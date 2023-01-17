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
    itp1_4_c();
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
