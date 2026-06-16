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
    let v1 = take_vector();
    let v2 = take_vector();

    let mut zippy = (0..n).map(|x| (v1[x] - 1, v2[x] - 1)).collect::<Vec<_>>();
    zippy.sort_by_key(|x| -((x.0 + x.1).abs()));

    println!(
        "{}",
        (0..n)
            .map(|i| if i % 2 == 0 { zippy[i].0 } else { -zippy[i].1 })
            .sum::<i64>()
    );
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
