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
    let n = take_int() as usize;
    let v = take_vector();
    let v2 = take_vector();
    let v = (0..n).map(|idx| v[idx] - v2[idx]).collect::<Vec<_>>();
    let m = *v.iter().max().unwrap();
    let mut ans = vec![];
    for i in 0..n {
        if v[i] == m {
            ans.push(i + 1)
        }
    }
    // print ans
    let out = ans .iter() .map(|x| x.to_string()) .collect::<Vec<String>>() .join(" ");
    println!("{}", ans.len());
    println!("{}", out);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
