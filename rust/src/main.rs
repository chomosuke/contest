#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
};

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next::<i128>();
    let m = sc.next::<i128>();
    let k = sc.next::<i128>();
    let m = m - n;
    let l = n - k;
    let r = k - 1;
    let mut x = (m as f64).sqrt() as i128;

    let s = min(r, l) as f64;
    let a = 0.5;
    let b = s + 0.5;
    let c = -(m as f64) - s * (s + 1.0) / 2.0;
    let x_s = (-b + (b*b - 4.0*a*c).sqrt()) / 2.0 / a;
    if s <= x_s {
        x = x_s as i128;
    }

    let x_rl = (m + l * (l + 1) / 2 + r * (r + 1) / 2) / n;
    if l <= x_rl && r <= x_rl {
        x = x_rl;
    }

    println!("{}", x + 1);
}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::{any::type_name, io::stdin, str::FromStr};

    pub struct Scanner {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
    }
    impl Scanner {
        pub fn new() -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: None,
            }
        }

        pub fn with_delimiters(delimiters: &[char]) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let token = loop {
                let front = self.tokens.pop_front();
                if let Some(token) = front {
                    break token;
                }
                self.receive_input();
            };
            token
                .parse::<T>()
                .unwrap_or_else(|_| panic!("input {} isn't a {}", token, type_name::<T>()))
        }

        pub fn next_line(&mut self) -> String {
            assert!(self.tokens.is_empty(), "You have unprocessed token");
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Failed to read.");
            line.pop();
            line
        }

        fn receive_input(&mut self) {
            let mut line = String::new();
            stdin().read_line(&mut line).expect("Failed to read.");
            if let Some(delimiters) = &self.delimiters {
                for token in line.split(|c| delimiters.contains(&c)) {
                    self.tokens.push_back(token.to_string());
                }
            } else {
                for token in line.split_whitespace() {
                    self.tokens.push_back(token.to_string());
                }
            }
        }
    }
}
#[allow(unused_imports)]
use scanner::*;
