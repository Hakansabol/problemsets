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

use std::time::{SystemTime, UNIX_EPOCH};

use template::*;
//=========================================================
//================== THE FUN BEGINS HERE ==================
//=========================================================

const DIFFERENCE: i32 = 100;
fn solve_iter() {
    let mut v: Vec<i32> = (0..1000000).collect();
    let mut out: Vec<i32> = ((DIFFERENCE)..(1000000 + DIFFERENCE)).collect();

    let stime = SystemTime::now();

    for _ in 0..DIFFERENCE {
        v = v.iter().enumerate().map(|(a, &x)| if a % 2 == 0 {x + 1} else {x}).collect();
    }
    let pr = &v[0..200];
    let out = pr
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);

    let etime = SystemTime::now();

    // print time
    let time_diff_sec = etime.duration_since(UNIX_EPOCH).unwrap();
    let a = stime.duration_since(UNIX_EPOCH).unwrap();
    let d = time_diff_sec - a;
    println!("millis total: {}", d.as_millis());
    println!("nanos per: {}", d.as_nanos() / DIFFERENCE as u128);
}

fn solve_forl() {
    let mut v: Vec<i32> = (0..1000000).collect();
    let mut out: Vec<i32> = ((DIFFERENCE)..(1000000 + DIFFERENCE)).collect();

    let stime = SystemTime::now();

    for i in 0..DIFFERENCE as usize {
        for j in 0..500000 {
            v[j * 2] += 1;
        }
    }
    let pr = &v[0..200];
    let out = pr
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);

    let etime = SystemTime::now();

    // print time
    let time_diff_sec = etime.duration_since(UNIX_EPOCH).unwrap();
    let a = stime.duration_since(UNIX_EPOCH).unwrap();
    let d = time_diff_sec - a;
    println!("millis total: {}", d.as_millis());
    println!("nanos per: {}", d.as_nanos() / DIFFERENCE as u128);
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve_iter();
        solve_forl();
        solve_iter();
        solve_forl();
    }
}
