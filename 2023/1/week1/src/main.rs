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
    //itp1_1_a();
    //itp1_1_b();
    //itp1_1_c();
    //itp1_1_d();
    //itp1_2_a();
    itp1_2_b();
}

fn itp1_2_b() {
    let line: Vec<i32> = read_line();
    let a: i32 = line[0];
    let b: i32 = line[1];
    let c: i32 = line[2];

    if a < b && b < c {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn itp1_2_a() {
    let line: Vec<i32> = read_line();
    let a: i32 = line[0];
    let b: i32 = line[1];

    if a < b {
        println!("a < b");
    } else if a > b {
        println!("a > b");
    } else {
        println!("a == b");
    }
}

fn itp1_1_d() {
    let line: Vec<i32> = read_line();
    let mut sec: i32 = line[0];

    let hour: i32 = sec / 3600;
    sec = sec % 3600;

    let minute: i32 = sec / 60;
    sec = sec % 60;

    println!("{}:{}:{}", hour, minute, sec);
}

fn itp1_1_c() {
    let line: Vec<i32> = read_line();
    let mut area: i32 = 1;
    let mut circle: i32 = 0;

    for e in line.iter() {
        area = area * e;
        circle = circle + (e * 2);
    }

    println!("{} {}", area, circle);
}

fn itp1_1_b() {
    let line: Vec<i32> = read_line();

    let num: i32 = line[0];

    let mut ans: i32 = 1;

    for _ in 1..=3 {
        ans = ans * num;
    }

    println!("{}", ans);
}

fn itp1_1_a() {
    println!("Hello World");
}
