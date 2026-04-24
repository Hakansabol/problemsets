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
//=========================================================
//================== THE FUN BEGINS HERE ==================
//=========================================================

fn solve() {
    let v = take_vector();
    let (mut t, mut h, u) = (v[0], v[1], v[2]);

    let mut ans: i64 = 3 * (t + h + u);
    if t > 0 && u > 0 {
        let cnt = std::cmp::min(t, u);
        t -= cnt;
        ans -= cnt * 2;
    }

    if t > 1 && h > 0 {
        let dubcnt = std::cmp::min(t / 2, h);
        t -= dubcnt * 2;
        h -= dubcnt;
        ans -= dubcnt * 2;
    }

    if t > 0 && h > 0 {
        t -= 1;
        ans -= 1;
    }

    if t > 1 {
        ans -= t - 1;
    }

    println!("{}", ans);
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
