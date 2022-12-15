use proconio::input;

fn main() {
    input! {
        n: i32,
    };

    let mut fact_num: i32 = 0;
    for i in 1..=n {
        fact_num = fact_num + i;
    }

    println!("{}", fact_num);
}
