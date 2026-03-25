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

fn take_int() -> i64 {
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
    let n = take_int();
    let tot = ((n + 1) * n) / 2;
    if tot % 2 == 0 {
        println!("YES");
        let mut left = tot / 2;
        let mut vprim = vec![];
        let mut vsec = vec![];
        for i in (1..=n).rev() {
            if i <= left {
                vprim.push(i);
                left -= i;
            } else {
                vsec.push(i);
            }
        }
        let ll = vprim.len();
        let rl = vsec.len();
        println!("{ll}");
        let ol = vprim
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{ol}");
        println!("{rl}");
        let ol = vsec
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{ol}");
    } else {
        println!("NO");
    }
}

pub fn main() {
    solve();
}
