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
    let allsum = v.iter().sum::<i64>();
    let mut v: Vec<(i64, i64)> = v.into_iter().zip(1i64..).collect();
    v.sort_by_key(|x| x.0);

    
}

pub fn main() {
    solve();
}
// if the array sum is odd, it is good
// if the array contains an odd number, ans <= 1
// if the array contains only even numbers, uhhhhhh
