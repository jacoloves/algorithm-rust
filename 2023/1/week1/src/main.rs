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
    itp1_9_d();
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

fn itp1_8_d() {
    let l: Vec<String> = read_line();
    let s = l[0].clone();

    let l2: Vec<String> = read_line();
    let p = l2[0].clone();

    let mut string_cnt = 0;
    loop {
        if s.len() == string_cnt {
            println!("No");
            break;
        }
        let mut comp_str = String::new();
        let mut cnt = 1;
        for c in s.chars() {
            if comp_str.len() == p.len() {
                break;
            }
            if cnt > s.len() - string_cnt {
                comp_str.push_str(&c.to_string());
            }
            cnt += 1;
        }

        cnt = 1;
        for c in s.chars() {
            if comp_str.len() == p.len() {
                break;
            }
            if cnt <= s.len() - string_cnt {
                comp_str.push_str(&c.to_string());
            }
            cnt += 1;
        }

        // judge
        if p == comp_str {
            println!("Yes");
            break;
        }

        string_cnt += 1;
    }
}

fn itp1_9_a() {
    let l: Vec<String> = read_line();
    let w = l[0].clone();

    let mut cnt = 0;
    loop {
        let l: Vec<String> = read_line();

        if l[0] == "END_OF_TEXT" {
            break;
        }

        for e in l {
            if w.to_lowercase() == e.to_lowercase() {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}

fn itp1_9_b() {
    let mut ans: Vec<String> = Vec::new();
    loop {
        let l: Vec<String> = read_line();
        // string
        let mut m = l[0].clone();

        if m == "-" {
            break;
        }

        // input num
        let l: Vec<i32> = read_line();
        let input_num = l[0];

        // input
        for i in 0..input_num {
            let mut comp_str: String = String::new();
            let l: Vec<i32> = read_line();
            let position = l[0];
            let mut cnt = 0;
            for c in m.chars() {
                cnt += 1;
                if cnt <= position {
                    continue;
                }

                comp_str.push_str(&c.to_string());
            }
            cnt = 0;
            for c in m.chars() {
                if cnt < position {
                    comp_str.push_str(&c.to_string());
                } else {
                    break;
                }
                cnt += 1;
            }

            m = comp_str.clone();
        }

        ans.push(m);
    }

    for e in ans {
        println!("{}", e);
    }
}

fn itp1_9_c() {
    let l: Vec<usize> = read_line();
    let n: usize = l[0];

    let mut tp = 0;
    let mut hp = 0;
    for _ in 0..n {
        let l: Vec<String> = read_line();

        let t: String = l[0].clone();
        let h: String = l[1].clone();

        if t > h {
            tp += 3;
        } else if t < h {
            hp += 3;
        } else {
            tp += 1;
            hp += 1;
        }
    }

    println!("{} {}", tp, hp);
}

fn itp1_9_d() {
    // string
    let l: Vec<String> = read_line();
    let mut s: String = l[0].clone();

    // input_num
    let l: Vec<usize> = read_line();
    let input_num: usize = l[0];

    let mut ans: Vec<String> = Vec::new();
    for _ in 0..input_num {
        let l: Vec<String> = read_line();
        let op: String = l[0].clone();
        match op.as_str() {
            // print
            "print" => {
                let first_num: usize = l[1].parse().unwrap();
                let second_num: usize = l[2].parse().unwrap();
                let mut cnt: usize = 0;
                let mut tmp_str: String = String::new();
                for c in s.chars() {
                    if first_num > cnt {
                        cnt += 1;
                        continue;
                    } else if second_num < cnt {
                        break;
                    }

                    tmp_str.push_str(&c.to_string());
                    cnt += 1
                }
                ans.push(tmp_str);
            }

            // replace
            "replace" => {
                let first_num: i32 = l[1].parse().unwrap();
                let second_num: i32 = l[2].parse().unwrap();
                let change_str: String = l[3].clone();
                let mut comp_str: String = String::new();
                let mut cnt: i32 = 0;
                let mut replace_cnt = 0;
                for c in s.chars() {
                    if first_num > cnt {
                        comp_str.push_str(&c.to_string());
                        cnt += 1;
                    } else if second_num < cnt {
                        comp_str.push_str(&c.to_string());
                        cnt += 1;
                    } else if replace_cnt != (second_num - first_num + 1) {
                        for rc in change_str.chars() {
                            comp_str.push_str(&rc.to_string());
                            replace_cnt += 1;
                        }
                        cnt += 1;
                    } else {
                        cnt += 1;
                    }
                }

                s = comp_str.clone();
            }

            // reverse
            _ => {
                let first_num: u32 = l[1].parse().unwrap();
                let second_num: u32 = l[2].parse().unwrap();
                let mut tmp_str: String = String::new();
                let mut tmp_str2: String = String::new();

                let mut cnt: u32 = 0;
                for c in s.chars() {
                    if first_num > cnt {
                        cnt += 1;
                    } else if second_num < cnt {
                        cnt += 1;
                    } else {
                        tmp_str.push_str(&c.to_string());
                        cnt += 1;
                    }
                }

                let reverse: String = tmp_str.chars().rev().collect::<String>();

                cnt = 0;
                let mut flg: bool = false;
                for c in s.chars() {
                    if first_num > cnt {
                        tmp_str2.push_str(&c.to_string());
                        cnt += 1;
                    } else if second_num < cnt {
                        tmp_str2.push_str(&c.to_string());
                        cnt += 1;
                    } else if !flg {
                        for rc in reverse.chars() {
                            tmp_str2.push_str(&rc.to_string());
                        }
                        flg = true;
                        cnt += 1;
                    } else {
                        cnt += 1;
                    }
                }

                s = tmp_str2.clone();
            }
        }
    }

    for s in ans.iter() {
        println!("{}", s);
    }
}
