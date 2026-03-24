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

fn probability_choose(n: i64, r: i64) -> i64 {
    let mut fact = 1i64;
    for i in r..=n {
        fact *= i;
    }
    let mut div = 1i64;
    for i in 1..=n - r {
        div *= i;
    }
    fact / div
}
fn solve(n: i64) -> i64 {
    let fixed_key = vec![0i64, 6i64, 28i64, 96i64, 252i64];
    if ((n as usize) <= fixed_key.len()) {
        fixed_key[(n - 1) as usize]
    } else {
        // ceeeec 234432
        // ecmmce 346643
        // emxxme 468864
        // emxxme 468864
        // ecmmce 346643
        // ceeeec 234432
        let mut ans = probability_choose(n * n, 2);
        let mut sub = 0i64;
        sub += 32;
        sub += (n - 3) * 4 * 4;
        sub += (n - 4) * 4 * 6;
        sub += (n - 4) * (n - 4) * 8;
        ans -= sub;
        ans
    }
}

pub fn main() {
    let t = take_int();
    for i in 1..=t {
        let a = solve(i as i64);
        println!("{a}");
    }
}
