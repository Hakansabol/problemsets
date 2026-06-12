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
    let (a, b, x) = (tv[0], tv[1], tv[2]);
    let mut lo = std::cmp::min(a, b);
    let mut hi = std::cmp::max(a, b);

    let mut ans = hi - lo;
    let mut bonus = 0;
    while hi + lo > 0 {
        if hi > lo {
            hi /= x;
        } else {
            lo /= x;
        }
        bonus += 1;
        ans = std::cmp::min((hi - lo).abs() + bonus, ans);
        // println!("{} {} {}", hi,lo,ans);
    }
    let mut ans = std::cmp::min((hi - lo).abs() + bonus, ans);
    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
