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
    let mut v = take_vector();

    v.sort();
    let mp = n as usize / 2;
    let m = v[n as usize / 2];

    let mut lr = (0, 0);
    for i in (0..mp).rev() {
        let a = v[i];
        if a == m {
            lr.0 += 1;
        } else {
            break;
        }
    }
    for i in mp + 1..n {
        let a = v[i];
        if a == m {
            lr.1 += 1;
        } else {
            break;
        }
    }

    println!("{}", std::cmp::max(mp - 0 - lr.0, n - 1 - mp - lr.1));
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
