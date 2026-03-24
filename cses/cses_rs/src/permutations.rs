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
    let n = take_int();

    let out = if n < 4 && n > 1 {
        String::from("NO SOLUTION")
    } else if n == 4 {
        String::from("2 4 1 3")
    } else {
        let mut ans = String::from("");
        let mut l = 1i32;
        let mut r = (n + 1) / 2 + 1;
        for i in 0..(n + 1) / 2 {
            if i > 0 {
                ans.push(' ');
            }
            ans.push_str(&(l + i).to_string());
            if r + i <= n {
                ans.push(' ');
                ans.push_str(&(r + i).to_string());
            };
        }
        ans
    };
    println!("{out}");
}

pub fn main() {
    solve()
}

// 1234567
// 1526374
