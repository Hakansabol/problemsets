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

    let mut dp: Vec<(i64, i64)> = vec![(0, 0); n + 2];

    for i in (0..n).rev() {
        let skp1 = v[i];
        let skp2 = skp1 + if i < n - 1 { v[i + 1] } else { 0 };
        dp[i] = (
            std::cmp::min(dp[i + 1].1, dp[i + 2].1),
            std::cmp::min(dp[i + 1].0 + skp1, dp[i + 2].0 + skp2),
        )
    }

    println!("{}", dp[0].1);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
