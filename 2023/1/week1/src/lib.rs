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

fn itp1_1_a() {
    println!("Hello World");
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

fn itp1_1_d() {
    let line: Vec<i32> = read_line();
    let mut sec: i32 = line[0];

    let hour: i32 = sec / 3600;
    sec = sec % 3600;

    let minute: i32 = sec / 60;
    sec = sec % 60;

    println!("{}:{}:{}", hour, minute, sec);
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

fn itp1_2_c() {
    let mut line: Vec<i32> = read_line();

    line.sort();

    let a: i32 = line[0];
    let b: i32 = line[1];
    let c: i32 = line[2];

    println!("{} {} {}", a, b, c);
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

fn itp1_3_a() {
    for _ in 1..=1000 {
        println!("Hello World");
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

fn itp1_3_d() {
    let line: Vec<i32> = read_line();
    let a: i32 = line[0];
    let b: i32 = line[1];
    let c: i32 = line[2];

    let mut ans: i32 = 0;

    for i in a..=b {
        if c % i == 0 {
            ans = ans + 1;
        }
    }

    println!("{}", ans);
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

fn itp1_4_b() {
    let line: Vec<f64> = read_line();

    let area: f64 = line[0].powf(2.0) * std::f64::consts::PI;
    let circle: f64 = 2.0 * std::f64::consts::PI * line[0];

    println!("{} {}", area, circle);
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

fn itp1_4_d() {
    let _: Vec<i32> = read_line();
    let line: Vec<i64> = read_line();

    let max_num: i64 = *line.iter().max().unwrap();
    let min_num: i64 = *line.iter().min().unwrap();
    let sum_num: i64 = line.iter().sum();

    println!("{} {} {}", min_num, max_num, sum_num);
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

fn itp1_6_b() {
    let mut trumps: Vec<Vec<usize>> = vec![vec![0; 13]; 4];

    let fl: Vec<i32> = read_line();
    let en: i32 = fl[0];

    let mut ic = 1;
    while ic <= en {
        let l: Vec<String> = read_line();
        let mut first: usize;
        let mut second: usize;

        match l[0].as_str() {
            "S" => {
                first = 0;
            }
            "H" => {
                first = 1;
            }
            "C" => {
                first = 2;
            }
            _ => {
                first = 3;
            }
        }

        second = l[1].parse().unwrap();

        trumps[first][second - 1] = 1;
        ic = ic + 1
    }

    for i in 0..4 {
        for j in 0..13 {
            if trumps[i][j] == 0 {
                match i {
                    0 => {
                        print!("S ",)
                    }
                    1 => {
                        print!("H ",)
                    }
                    2 => {
                        print!("C ",)
                    }
                    _ => {
                        print!("D ",)
                    }
                }

                println!("{}", j + 1);
            }
        }
    }
}

fn itp1_6_c() {
    let mut univ: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 10]; 3]; 4];

    let fl: Vec<i32> = read_line();
    let en: i32 = fl[0];

    let mut ic = 1;
    while ic <= en {
        let l: Vec<i32> = read_line();
        let first: i32 = l[0] - 1;
        let second: i32 = l[1] - 1;
        let third: i32 = l[2] - 1;
        let ele: i32 = l[3];

        univ[first as usize][second as usize][third as usize] =
            univ[first as usize][second as usize][third as usize] + ele as usize;
        if univ[first as usize][second as usize][third as usize] >= 10 {
            univ[first as usize][second as usize][third as usize] = 9
        }
        ic = ic + 1;
    }

    for i in 0..4 {
        for j in 0..3 {
            for k in 0..10 {
                print!(" {}", univ[i][j][k]);
            }
            println!();
        }
        if i != 3 {
            println!("####################");
        }
    }
}

fn itp1_6_d() {
    let fl: Vec<i32> = read_line();
    let first: i32 = fl[0];
    let second: i32 = fl[1];

    let mut vector1: Vec<Vec<usize>> = vec![vec![0; second as usize]; first as usize];

    let mut vector2: Vec<usize> = vec![0; second as usize];

    for i in 0..first {
        let l: Vec<usize> = read_line();
        for j in 0..second {
            vector1[i as usize][j as usize] = l[j as usize];
        }
    }

    for i in 0..second {
        let l: Vec<usize> = read_line();
        vector2[i as usize] = l[0];
    }

    for i in 0..first {
        let mut ans: usize = 0;
        for j in 0..second {
            ans = ans + (vector1[i as usize][j as usize] * vector2[j as usize]);
        }
        println!("{}", ans);
    }
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

fn itp1_10_a() {
    let l: Vec<f64> = read_line();

    let x1: f64 = l[0];
    let y1: f64 = l[1];
    let x2: f64 = l[2];
    let y2: f64 = l[3];

    let ans: f64 = (x1 - x2).powi(2) + (y1 - y2).powi(2);

    println!("{}", ans.sqrt());
}

pub fn itp1_10_b() {
    let l: Vec<f64> = read_line();

    let a: f64 = l[0];
    let b: f64 = l[1];
    let c: f64 = l[2];

    println!("{} {} {}", a, b, c);

    let tmp_c: f64 = a.powf(2.0_f64) + b.powf(2.0_f64) - (2.0_f64 * a * b * c.cos());
    let line_c: f64 = tmp_c.sqrt();

    let s: f64 = (a * b * c.sin()) / 2.0_f64;
    let ll: f64 = a + b + line_c;
    let h: f64 = b * c.sin();

    println!("{}", s);
    println!("{}", ll);
    println!("{}", h);
}
