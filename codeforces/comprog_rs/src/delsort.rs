// https://codeforces.com/problemset/problem/2200/B

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

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap()
}

fn take_vector() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    return arr;
}

fn take_string() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec:Vec<char> = input.trim().chars().collect();
    return vec;
}
fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}

fn solve() {
    let n = take_int();
    let v = take_vector();

    let mut out = 1;
    let mut decr = 0;
    let mut l = v[0];
    for a in v {
        if a < l {
            decr += 1;
        }
        l = a;
    }
    if decr > 0 {
        out = 1;
    }
    else {
        out = n;
    }

    println!("{out}");
}


pub fn main() {
    let t = take_int();
    for _ in 0..t { solve(); }
}
