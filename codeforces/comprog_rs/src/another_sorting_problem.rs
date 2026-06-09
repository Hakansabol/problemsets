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

fn solve() {
    let n = take_int() as usize;
    let v = take_vector();
    let mut hv = 0;

    for i in 1..v.len() {
        let diff = (v[i - 1] - v[i]);
        if diff > 0 {
            hv = std::cmp::max(diff, hv)
        }
    }

    let mut up = false;
    for i in 1..v.len() {
        let diff = v[i] - v[i - 1];
        if diff < 0 {
            if up {
                println!("{}", "NO");
                return;
            }
            up = true;
        }
        if diff >= hv {
            up = false;
        }
    }

    println!("{}", "YES");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
