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
    let (n, k) = (v[0] as usize, v[1]);
    let a = take_vector();
    let b = take_vector();

    let mut ans = i64::min_value();
    let mut grid = vec![];
    grid.push(vec![0i64; n]);
    grid.push(vec![0i64; n]);

    // generate grid[0] , linear max subarray alg
    for i in 0..n {
        grid[0][i] = std::cmp::max(if i == 0 { 0 } else { grid[0][i - 1] }, 0) + a[i];
    }
    // generate grid[1] , very slight dp + max subarray alg again
    for i in 0..n {
        grid[1][i] = if i == 0 {
            a[0] + b[0]
        } else {
            std::cmp::max(grid[1][i - 1] + a[i] as i64, grid[0][i] + b[i])
        }
    }

    // get the best answer, only check grid[1] if k%2==1
    for i in 0..grid[0].len() {
        ans = std::cmp::max(ans, grid[0][i]);
        if k % 2 == 1 {
            ans = std::cmp::max(ans, grid[1][i]);
        }
    }
    
    // print the answer
    println!("{}", ans);
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
// two track
// track 0: dont take, ever
// track 1: taken
// max between track[0][n-1] + b[n] and track[1][n-1]
// then add a, ofc
