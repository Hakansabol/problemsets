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

use std::cmp::{max, min};

use template::*;

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();
    let mut ans = 0i64;

    let mut g = vec![0i64; n + 1];
    for i in 1..=n {
        g[i] = g[i - 1];
        if v[i] < (i) as i64 {
            ans += g[min(max(0, (v[i] - 1)) as usize, n - 1)];
            g[i] += 1;
        }
    }

    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
