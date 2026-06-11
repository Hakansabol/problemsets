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

use template::*;

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();

    let mut dp = vec![1; n];
    dp.push(0);
    for i in (0..n).rev() {
        let mut nv = 1000000000;
        if i + 1 < n {
            nv = std::cmp::min(nv, dp[i + 1] + 1);
        }
        if (i as i64) + v[i] < n as i64 {
            nv = std::cmp::min(nv, dp[(i + v[i] as usize + 1) as usize])
        }
        if nv < 1000000000 {
            dp[i] = nv;
        }
    }
    println!("{}", dp[0]);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
