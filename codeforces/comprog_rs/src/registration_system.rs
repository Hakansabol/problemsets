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

use std::{collections::HashMap, hash::Hash, io::stdin, ops::DerefMut};

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

pub fn main() {
    let mut dict: HashMap<String, i32> = HashMap::new();

    let i = take_int();
    for _ in 0..i {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s = s.split_whitespace().next().unwrap().to_owned();
        let mut out = s.clone();

        let mut iss = dict.entry(s).or_insert(-1);
        *iss += 1;
        if *iss > 0 {
            out.push_str(&*iss.to_string());

            println!("{}", out);
        } else {
            println!("{}", "OK");
        }
    }
}
