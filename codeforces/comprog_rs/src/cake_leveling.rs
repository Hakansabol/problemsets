// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use std::i64;

use template::*;

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();
    let mut ans = vec![0i64; n];

    let mut run: i64 = 0;
    let mut fail: i64 = i64::MAX;
    for i in 0..n {
        let a = v[i];

        run += a;
        let mv = run / ((i + 1) as i64);
        fail = std::cmp::min(fail, mv);
        ans[i] = std::cmp::min(fail, mv);
    }

    let out = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out)
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
