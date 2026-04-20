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

use std::{collections::HashMap, hash::Hash};

use template::*;
//=========================================================
//================== THE FUN BEGINS HERE ==================
//=========================================================

fn solve() {
    let tv = take_vector();
    let (n, k) = (tv[0], tv[1]);
    let v = take_vector();

    let mut utensils_remaining = HashMap::new();
    for a in v {
        *utensils_remaining.entry(a).or_insert(0) += 1;
    }
    let mut most_utensils = *utensils_remaining.values().max().unwrap();
    most_utensils = k * ((most_utensils - 1) / k + 1);

    let ans = (most_utensils * utensils_remaining.iter().len() as i32) - n;
    println!("{}", ans);
}

pub fn main() {
    solve();
}
