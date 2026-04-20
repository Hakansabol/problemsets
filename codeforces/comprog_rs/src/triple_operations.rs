/* This template is made by Naman Garg <naman.rustp@gmail.com> GitHub : https://github.com/namanlp GitLab : https://gitlab.com/namanlp Website : https://rustp.org You can visit https://rustp.org/basic-programs/basic-template/ for understanding the template Feel free to copy the template, but not the solutions :D Thank You */ #![allow(unused)] use std::io::stdin; fn take_int() -> i32 { let mut input = String::new(); stdin().read_line(&mut input).unwrap(); return input.trim().parse().unwrap(); } fn take_vector() -> Vec<u32> { let mut input = String::new(); stdin().read_line(&mut input).unwrap(); let arr: Vec<u32> = input .trim() .split_whitespace() .map(|x| x.parse().unwrap()) .collect(); return arr; } fn take_string() -> Vec<char> { let mut input = String::new(); stdin().read_line(&mut input).unwrap(); let vec: Vec<char> = input.trim().chars().collect(); return vec; } fn to_string(vec: Vec<char>) -> String { return vec.iter().collect::<String>(); }

fn solve() {
    let v = take_vector();
    let (l, r) = (v[0], v[1]);

    println!("{}", (0..=14).into_iter().map(|x| (std::cmp::min(3u32.pow(x+1)-1,r).saturating_sub(std::cmp::max(3u32.pow(x)-1,l-1))) * (x+1)).sum::<u32>() + l.ilog(3)+1);
}

pub fn main() {
    let t = take_int();
    for _ in 0..t {
        solve();
    }
}
