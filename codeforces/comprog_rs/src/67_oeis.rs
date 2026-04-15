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

/// ztemplate: Sieve of Eratosthenes [first n primes]
/// cnt: the number of primes to find, starting with `1`.
fn eratosthenes(cnt: usize) -> Vec<u64> {
    assert!(cnt <= 1001000, "cnt too large for the sieve (> 1.001M)");
    let mut sieve: Vec<bool> = vec![false; cnt * 16]; // the sieve could be a vector of u8 for memory efficiency.
    let mut basin: Vec<u64> = vec![]; // the basin is the output where the primes fall into.
    let mut point: u64 = 2; // The point is the point that the sieve is currently at.
    while (basin.len() < cnt) {
        // if this item is not flagged
        if !sieve[point as usize] {
            basin.push(point);
            // update the sieve
            let mut sieve_point = point;
            while sieve_point < sieve.len() as u64 {
                sieve[sieve_point as usize] = true;
                sieve_point += point;
            }
        }
        // next number
        point += 1;
    }
    basin
}

pub fn main() {
    let t = take_int();
    let mut cases = vec![];
    for _ in 0..t {
        cases.push(take_int());
    }
    let biggest = cases.iter().max();
    let primes = eratosthenes((biggest.unwrap() + 5) as usize);
    for a in cases {
        let mut build = vec![];
        for i in 0..a as usize {
            build.push(primes[i] * primes[i + 1]);
        }
        let out = build
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{out}");
    }
}
    /// ztemplate: Sieve of Eratosthenes [first n primes]
    /// cnt: the number of primes to find, starting with `1`.
    fn eratosthenes(cnt: usize) -> Vec<u64> {
        assert!(cnt <= 1001000, "cnt too large for the sieve (> 1.001M)");
        let mut sieve: Vec<bool> = vec![false; cnt * 16]; // the sieve could be a vector of u8 for memory efficiency.
        let mut basin: Vec<u64> = vec![]; // the basin is the output where the primes fall into.
        let mut point: u64 = 2; // The point is the point that the sieve is currently at.
        while (basin.len() < cnt) {
            // if this item is not flagged
            if !sieve[point as usize] {
                basin.push(point);
                // update the sieve
                let mut sieve_point = point;
                while sieve_point < sieve.len() as u64 {
                    sieve[sieve_point as usize] = true;
                    sieve_point += point;
                }
            }
            // next number
            point += 1;
        }
        basin
    }
z
