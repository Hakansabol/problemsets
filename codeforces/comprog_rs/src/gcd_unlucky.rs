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
    let v1 = take_vector();
    let v2 = take_vector();

    // validate l>r and r>l
    for i in 0..v1.len() - 1 {
        if v1[i] % v1[i + 1] > 0 {
            println!("{}", "NO");
            eprintln!("v1 err");
            return;
        }
        if v2[n - i - 1] % v2[n - i - 2] > 0 {
            println!("{}", "NO");
            eprintln!("v2 err");
            return;
        }
    }

    let mut switch = false;
    for i in 0..v1.len() {
        if v1[i] < v2[i] {
            switch = true;
        } else if switch == true && v1[i] > v2[i] {
            println!("{}", "NO");
        }
    }

    println!("{}", "YES");
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
