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

use std::collections::HashMap;

use template::*;

fn solve() {
    let n = take_int();
    let v = take_vector();

    let mut s = v.clone();
    s.sort();
    let pivot = s[n as usize / 2];

    let v: Vec<i32> = v
        .iter()
        .map(|&x| {
            if x > pivot {
                1
            } else if x < pivot {
                -1
            } else {
                0
            }
        })
        .collect();
    let mut ans = 0;

    let ans = dp(&v, n, 0, 0, 0, 0);

    println!("{}", ans);
}
fn dp(v: &Vec<i32>, n: i32, l: i32, r: i32, pcount: i32, offset: i32) -> i32 {
    if r == n {
        return if l == r { 0 } else { -1000000 };
    }
    let mut ans = -100000;

    let offset = offset + v[r as usize];
    let mut pcount = pcount;
    if v[r as usize] == 0 {
        pcount += 1;
    };

    if pcount > offset.abs() && l != r {
        ans = std::cmp::max(ans, dp(v, n, r + 1, r + 1, 0, 0) + 1);
    }
    ans = std::cmp::max(ans, dp(v, n, l, r + 1, pcount, offset));
    ans
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
