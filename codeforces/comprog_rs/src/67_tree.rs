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
    let v = take_vector();
    let (mut x, mut y) = (v[0], v[1]);
    let n = x + y;
    if n % 2 == 0 {
        x -= 1;
    } else {
        y -= 1;
    }
    if x < 0 || y < 0 || x > y {
        println!("NO");
        return;
    }

    let singles = y - x;
    let doubles = x;

    println!("YES");
    let mut last = 2;
    for _ in 0..doubles {
        println!("{} {}", 1, last);
        println!("{} {}", last, last + 1);
        last += 2;
    }
    for _ in 0..singles {
        println!("{} {}", 1, last);
        last += 1;
    }
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}

// i must always be less than or equal to j
// you can remove an i or j at the start for node 1 because it will never change
// just place all the j nodes at the start as children of 1
// then just move `i` of them as children of the j nodes and you win!
