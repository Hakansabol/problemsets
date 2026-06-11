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
    let n = take_int() as i64;

    let mut palindromes = vec![];
    for i in 1..=n {
        let ip = i.to_string().chars().rev().collect::<String>().parse::<i64>().unwrap() == i;
        if ip {palindromes.push(i);}
    }
    println!("{:?}", palindromes);

    let mut dp = vec![0;n as usize + 1];
    dp[0] = 1;
    for &a in &palindromes {dp[(a) as usize] += 1;}
    println!("{:?}", dp);
    for i in 1..=n {
        for &a in &palindromes {
            if a * 2 > i {
                break;
            }
            dp[i as usize] += dp[(a) as usize];
            dp[i as usize] = dp[i as usize] % 1000000007;
        }
    }

    println!("{:?}", dp);
    println!("{}", dp[n as usize]);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
