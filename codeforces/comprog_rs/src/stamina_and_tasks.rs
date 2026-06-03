// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<f64>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;

fn solve() {
    let n = take_int() as usize;
    let mut v = vec![];
    for a in 0..n {
        v.push(take_vector());
    }

    let mut grid = vec![0f64; n as usize];
    grid[n - 1] = v[n - 1][0];
    for i in (0..n as usize - 1).rev() {
        grid[i] = grid[i + 1].max(v[i][0] + (grid[i + 1] * (1f64 - (v[i][1] / 100f64))));
    }
    println!("{}", grid.into_iter().fold(0f64, |a, x| x.max(a)));
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
