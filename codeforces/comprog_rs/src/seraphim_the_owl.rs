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
    let tv = take_vector();
    let (n, m) = (tv[0] as usize, tv[1] as usize);
    let v1 = take_vector();
    let v2 = take_vector();

    let mut dp = vec![(0i64, 0i64); n + 2];
    // track `.0` : in that place
    // track `.1` : skipped that place

    for i in (1..=n).rev() {
        dp[i].0 = std::cmp::min(dp[i + 1].0, dp[i + 1].1) + v1[i - 1];
        dp[i].1 = std::cmp::min(dp[i + 1].0, dp[i + 1].1) + v2[i - 1];
    }

    let mut ans = i64::MAX;
    for i in 1..=m {
        ans = std::cmp::min(ans, dp[i].0);
    }
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
