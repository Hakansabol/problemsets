/*
This template is made by Naman Garg <naman.rustp@gmail.com>
GitHub : https://github.com/namanlp
GitLab : https://gitlab.com/namanlp
Website : https://rustp.org

You can visit https://rustp.org/basic-programs/basic-template/
for understanding the template

Feel free to copy the template, but not the solutions :D
Thank You
 */

#![allow(unused)]

use std::io::stdin;

fn take_int() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn take_vector() -> Vec<i32> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return arr;
}

fn take_string() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec: Vec<char> = input.trim().chars().collect();
    return vec;
}
fn to_string(vec: Vec<char>) -> String {
    return vec.iter().collect::<String>();
}

fn solve() {
    let v = take_vector();
    let x = v[0] as i64;
    let y = v[1] as i64;

    let sqv = std::cmp::max(x, y) - 1i64;
    let sq = sqv as i64 * sqv as i64;
    let primr = sqv % 2 == 1; // is the right the start pos?
    let p = if primr { y } else { x };
    let s = if primr { x } else { y };

    let mut ans = sq + sqv + 1;
    let diff = sqv - std::cmp::min(x, y) + 1;
    ans += if primr ^ (y >= x) { diff } else { -diff };
    println!("{ans}");
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
