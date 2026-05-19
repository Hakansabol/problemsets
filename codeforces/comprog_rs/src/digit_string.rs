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
    let s = take_string();

    let ans = s.clone().iter().filter(|x| *x == &'4').count() as i32;
    let s = s.into_iter().filter(|x| *x != '4').collect::<Vec<char>>();
    let n = s.len();
    if n == 0 {
        println!("{}", ans);
        return;
    }

    let mut twos = s.clone().iter().filter(|x| *x == &'2').count() as i32;
    let mut odds = 0;
    let mut ret = 1000000;
    for i in 0..=n {
        ret = std::cmp::min(ret, ans + twos + odds);
        if i < n {
            if s[i] == '2' {
                twos -= 1;
            } else {
                odds += 1;
            }
        }
    }
    println!("{}", ret);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
