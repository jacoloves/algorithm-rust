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
    itp1_4_b();
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
