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
    let n = take_int();
    let mut v = take_vector();

    v.sort();

    let mut bonus = 0i64;
    let mut ans = 0i64;
    let mut big: i64 = v.iter().sum();
    if big < 3 {
        println!("{}", 0);
        return;
    }

    let mut cnt = 0;
    for a in v {
        if a >= 2 {
            cnt += 1;
            ans += a;
        }
        if a >= 4 {
            ans += (a - 2) / 2;
        }
    }
    println!("{}", std::cmp::min(ans + if cnt == 1 { 1 } else { 0 }, big));
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}

// 1010101
// 110022122
