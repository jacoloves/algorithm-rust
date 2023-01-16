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
    //itp1_2_b();
    //itp1_2_c();
    //itp1_2_d();
    //itp1_3_a();
    //itp1_3_b();
    itp1_3_c();
}

fn itp1_3_c() {
    let mut av: Vec<i32> = Vec::new();
    let mut bv: Vec<i32> = Vec::new();
    loop {
        let line: Vec<i32> = read_line();
        if line[0] == 0 && line[1] == 0 {
            break;
        }

        av.push(line[0]);
        bv.push(line[1]);
    }

    for i in 0..av.len() {
        if av[i] > bv[i] {
            println!("{} {}", bv[i], av[i]);
        } else {
            println!("{} {}", av[i], bv[i]);
        }
    }
}

fn itp1_3_b() {
    let mut i: i32 = 1;

    let mut ans: Vec<String> = Vec::new();
    loop {
        let line: Vec<i32> = read_line();
        if line[0] == 0 {
            break;
        }
        let ele: String = format!("case {}: {}", i, line[0]);
        ans.push(ele);
        i = i + 1;
    }

    for e in ans.iter() {
        println!("{}", e);
    }
}

fn itp1_3_a() {
    for _ in 1..=1000 {
        println!("Hello World");
    }
}

fn itp1_2_d() {
    let line: Vec<i32> = read_line();

    let w: i32 = line[0];
    let h: i32 = line[1];
    let x: i32 = line[2];
    let y: i32 = line[3];
    let r: i32 = line[4];

    if w < x + r || h < y + r || 0 > x - r || 0 > y - r {
        println!("No");
    } else {
        println!("Yes");
    }
}

fn itp1_2_c() {
    let mut line: Vec<i32> = read_line();

    line.sort();

    let a: i32 = line[0];
    let b: i32 = line[1];
    let c: i32 = line[2];

    println!("{} {} {}", a, b, c);
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
