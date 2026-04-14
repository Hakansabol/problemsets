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
    let n = take_int() as u32;
    let s = take_string();

    let oc = s.iter().map(|x| ((*x as u8) - b'0') as u32).sum::<u32>();

    if oc % 2 == 0 {
        println!("{}", oc);
        let ones = s.iter().enumerate().filter(|x| *x.1 == '1').map(|x| x.0 + 1usize).collect::<Vec<usize>>().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        if oc > 0 {
            println!("{}", ones);
        }
    } else if (n - oc) % 2 == 1 {
        println!("{}", n-oc);
        let out = s.iter().enumerate().filter(|x| *x.1 == '0').map(|x| (x.0+1).to_string()).collect::<Vec<String>>().join(" ");

        println!("{}", out);
    } else {
        println!("-1");
    }
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
