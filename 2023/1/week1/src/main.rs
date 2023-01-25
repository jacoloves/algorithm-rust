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

#[derive(Debug)]
struct Dice {
    one_surface: u32,
    two_surface: u32,
    three_surface: u32,
    four_surface: u32,
    five_surface: u32,
    six_surface: u32,
    top_surface: u32,
}

impl Dice {
    fn new(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> Self {
        Dice {
            one_surface: a,
            two_surface: b,
            three_surface: c,
            four_surface: d,
            five_surface: e,
            six_surface: f,
            top_surface: a,
        }
    }

    fn dice_change(&mut self, azimuth: String) {
        for c in azimuth.chars() {
            let tmp_one: u32 = self.one_surface;
            let tmp_two: u32 = self.two_surface;
            let tmp_three: u32 = self.three_surface;
            let tmp_four: u32 = self.four_surface;
            let tmp_five: u32 = self.five_surface;
            let tmp_six: u32 = self.six_surface;

            match c {
                'E' => {
                    self.one_surface = tmp_four;
                    self.three_surface = tmp_one;
                    self.six_surface = tmp_three;
                    self.four_surface = tmp_six;
                    self.top_surface = self.one_surface;
                }
                'W' => {
                    self.one_surface = tmp_three;
                    self.three_surface = tmp_six;
                    self.six_surface = tmp_four;
                    self.four_surface = tmp_one;
                    self.top_surface = self.one_surface;
                }
                'N' => {
                    self.one_surface = tmp_two;
                    self.five_surface = tmp_one;
                    self.six_surface = tmp_five;
                    self.two_surface = tmp_six;
                    self.top_surface = self.one_surface;
                }
                'S' => {
                    self.one_surface = tmp_five;
                    self.five_surface = tmp_six;
                    self.six_surface = tmp_two;
                    self.two_surface = tmp_one;
                    self.top_surface = self.one_surface;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    itp1_11_a();
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

fn itp1_11_a() {
    let l: Vec<u32> = read_line();

    let mut d: Dice = Dice::new(l[0], l[1], l[2], l[3], l[4], l[5]);

    let l: Vec<String> = read_line();

    d.dice_change(l[0].clone());

    println!("{}", d.top_surface);
}
