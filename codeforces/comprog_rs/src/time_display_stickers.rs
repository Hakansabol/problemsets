// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<i32>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;
//=========================================================
//================== THE FUN BEGINS HERE ==================
//=========================================================

fn solve() {
    let s = take_string();
    let mut cnt = vec![0; 10];

    for a in s {
        cnt[((a as u8) - b'0') as usize] += 1;
    }

    let zc = cnt[0];
    let oc = cnt[1];
    let trashc: i32 = cnt[6..=9].iter().sum();
    let midc: i32 = cnt[2..=5].iter().sum();
    let mut nums = vec!{zc,oc,trashc,midc};
    let mut ans = 0;

    while *nums.clone().iter().min().unwrap() > 0 {
        nums = nums.iter().map(|x| x - 1).collect();
        ans += 1;
    }
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
