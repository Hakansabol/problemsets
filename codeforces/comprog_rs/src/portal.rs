// This template is adapted from Naman Garg's <naman.rustp@gmail.com> at https://rustp.org/basic-programs/basic-template/
#[rustfmt::skip]
#[allow(clippy::all, unused)]
mod template {
    use std::io::stdin;
    pub fn take_int() -> i32{std::io::stdin().lines().next().unwrap().unwrap().trim().parse().unwrap()}
    pub fn take_vector() -> Vec<usize>{std::io::stdin().lines().next().unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect()}
    pub fn take_string() -> Vec<char>{std::io::stdin().lines().next().unwrap().unwrap().trim().chars().collect()}
    pub fn to_string(vec:Vec<char>) -> String{return vec.iter().collect::<String>();}
}

use template::*;

fn solve() {
    let v = take_vector();
    let (n, x, y) = (v[0], v[1], v[2]);
    let v = take_vector();

    let mut lv = vec![];
    let mut rv = vec![];

    let mut one_in_left = false;
    for i in 0..x {
        lv.push(v[i]);
        if v[i] == 1 {
            one_in_left = true;
        }
    }
    for i in x..y {
        rv.push(v[i]);
    }
    for i in y..n {
        lv.push(v[i]);
        if v[i] == 1 {
            one_in_left = true;
        }
    }

    // order inner portal
    let min_inner = rv.iter().min().unwrap().clone(); // minimum value in center bloc
    let index_shift = rv.iter().position(|x| x == &min_inner).unwrap();
    // shift outer all the way to the right, then push to the left as long as element 0 is less than
    //  min inner
    let mut outer_shift: usize = 0;
    while lv.len() > outer_shift && (lv[outer_shift] < min_inner) {
        outer_shift += 1;
    }

    // print output
    // print the first outer_shift items of lv, then rv, then the rest of lv.
    let mut build = String::from("");
    for i in 0..outer_shift {
        build.push_str(&lv[i].to_string());
        build.push(' ');
    }
    for i in index_shift..rv.len() {
        build.push_str(&rv[i].to_string());
        build.push(' ');
    }
    for i in 0..index_shift {
        build.push_str(&rv[i].to_string());
        build.push(' ');
    }
    for i in outer_shift..lv.len() {
        build.push_str(&lv[i].to_string());
        build.push(' ');
    }
    println!("{}", build);
}

pub fn main() {
    for _ in 0..take_int() {
        solve();
    }
}
