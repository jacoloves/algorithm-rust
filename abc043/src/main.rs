use proconio::input;

fn main() {
    b();
}

fn b() {
    input! {
        s: String,
    };

    let mut ans: Vec<char> = Vec::new();

    for i in s.as_str().chars() {
        if i == '0' {
            ans.push('0')
        } else if i == '1' {
            ans.push('1')
        } else {
            if ans.is_empty() {
            } else {
                ans.pop();
            }
        }
    }

    for i in ans {
        print!("{}", i);
    }
    println!();
}

#[allow(dead_code)]
fn a() {
    input! {
        n: i32,
    };

    let mut fact_num: i32 = 0;
    for i in 1..=n {
        fact_num = fact_num + i;
    }

    println!("{}", fact_num);
}
