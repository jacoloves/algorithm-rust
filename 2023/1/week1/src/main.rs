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
    itp1_10_d();
}

fn itp1_10_c() {
    let mut ans: Vec<f64> = Vec::new();
    loop {
        let l: Vec<f64> = read_line();
        let n: f64 = l[0];

        if n == 0_f64 {
            break;
        }

        let l: Vec<f64> = read_line();
        let mut sum: f64 = 0_f64;

        // avarage
        for e in l.iter() {
            sum += e;
        }

        let avg: f64 = sum / n;

        // sd
        let mut sum_sd: f64 = 0_f64;

        for e in l.iter() {
            let sd: f64 = (e - avg).powi(2);
            sum_sd += sd;
        }

        let disp: f64 = sum_sd / n;

        ans.push(disp.sqrt());
    }

    for e in ans.iter() {
        println!("{}", e);
    }
}

fn itp1_10_d() {
    let l: Vec<usize> = read_line();
    let p: usize = l[0];

    let xv: Vec<f64> = read_line();
    let yv: Vec<f64> = read_line();

    // manhattan distance
    let mut manh_dis: f64 = 0_f64;

    for i in 0..p {
        manh_dis += (xv[i] - yv[i]).abs();
    }

    // Euclid distance 1
    let mut euc_dis1: f64 = 0_f64;

    for i in 0..p {
        euc_dis1 += ((xv[i] - yv[i]).abs()).powi(2);
    }

    let euc_dis_ans1: f64 = euc_dis1.sqrt();

    // Euclid distance 2
    let mut euc_dis2: f64 = 0_f64;

    for i in 0..p {
        euc_dis2 += ((xv[i] - yv[i]).abs()).powi(3);
    }

    let euc_dis_ans2: f64 = euc_dis2.powf(1.0 / 3.0);

    // chebyshev distance
    let mut che_dis_v: Vec<f64> = Vec::new();

    for i in 0..p {
        che_dis_v.push((xv[i] - yv[i]).abs());
    }

    let mut che_dis: f64 = 0_f64;

    for e in che_dis_v.iter() {
        if e > &che_dis {
            che_dis = *e;
        }
    }

    println!("{}", manh_dis);
    println!("{}", euc_dis_ans1);
    println!("{}", euc_dis_ans2);
    println!("{}", che_dis);
}
