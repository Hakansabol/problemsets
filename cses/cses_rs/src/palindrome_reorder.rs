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

fn solve() -> String {
    let s = take_string();
    let mut chars = vec![0i32; 27];
    for a in s {
        chars[((a as u8) - ('A' as u8)) as usize] += 1;
    }
    let mut center = 26;
    let mut ans: String = String::from("");
    for i in 0..26 {
        // find center, bail if there isnt a valid one
        let a = chars[i];
        if a % 2 == 1 {
            if center != 26 {
                return String::from("NO SOLUTION");
            }
            center = i;
        } else {
            let c: char = ('A' as u8 + i as u8) as char;
            ans.push_str(&(c.to_string()).repeat((a / 2) as usize));
        }
    }
    let mut out = ans.clone();
    out.push_str(
        &(('A' as u8 + center as u8) as char)
            .to_string()
            .repeat(chars[center as usize] as usize),
    );
    out.push_str(&ans.chars().into_iter().rev().collect::<String>());
    out
}

pub fn main() {
    let o = solve();
    println!("{o}");
}

