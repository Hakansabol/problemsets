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

use std::collections::HashSet;

use template::*;

fn solve() {
    let t = take_vector();
    let (n, k) = (t[0], t[1]);
    let mut v = take_vector();

    if k > 2 {
        println!("{}", 0);
        return;
    }

    let mut vb = Vec::with_capacity(4002001);

    for i in 0..v.len() {
        let a = v[i];
        for j in i + 1..v.len() {
            let b = v[j];
            vb.push((a - b).abs());
        }
    }

    let mut ans = *std::cmp::min(v.iter().min(), vb.iter().min()).unwrap();
    v.sort();
    vb.sort();

    if k == 2 {
        let mut idx = 0;
        for a in vb {
            while a > v[idx + 1] && idx < v.len() - 2 {
                idx += 1;
            }
            ans = std::cmp::min(
                ans,
                std::cmp::min((v[idx] - a).abs(), (v[idx + 1] - a).abs()),
            )
        }
    }

    println!("{}", ans);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
