#![allow(
    unused_imports,
    dead_code,
    clippy::needless_range_loop,
    clippy::comparison_chain
)]
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap},
};

fn possible(s: &mut [Option<u8>], i: usize) -> bool {
    if i == s.len() {
        return true;
    }
    let prev = s[i];
    let it = if let Some(c) = s[i] { c..=c } else { 0..=1 };
    for c in it {
        s[i] = Some(c);
        let p5_before = i >= 4 && s[i - 4] == s[i] && s[i - 3] == s[i - 1];
        let p6_before = i >= 5 && s[i - 5] == s[i] && s[i - 4] == s[i - 1] && s[i - 3] == s[i - 2];
        let p5_after = i + 4 < s.len()
            && s[i + 4].is_some()
            && s[i + 3].is_some()
            && s[i + 1].is_some()
            && s[i + 4] == s[i]
            && s[i + 3] == s[i + 1];
        let p6_after = i + 5 < s.len()
            && s[i + 5].is_some()
            && s[i + 4].is_some()
            && s[i + 3].is_some()
            && s[i + 2].is_some()
            && s[i + 1].is_some()
            && s[i + 5] == s[i]
            && s[i + 4] == s[i + 1]
            && s[i + 3] == s[i + 2];
        if !p5_before && !p6_before && !p5_after && !p6_after && possible(s, i + 1) {
            // no extra palindromes formed and rest is possible
            return true;
        }
    }
    s[i] = prev;
    false
}

fn solve(sc: &mut Scanner) {
    sc.next::<usize>();
    let mut s = sc
        .next_line()
        .into_bytes()
        .into_iter()
        .map(|b| match b {
            b'0' => Some(0u8),
            b'1' => Some(1u8),
            b'?' => None,
            _ => panic!(),
        })
        .collect::<Vec<_>>();
    if possible(&mut s, 0) {
        println!(" POSSIBLE");
    } else {
        println!(" IMPOSSIBLE");
    }
}

fn main() {
    let mut sc = Scanner::new();
    let test_cases = sc.next::<usize>();
    for case_number in 1..=test_cases {
        print!("Case #{}:", case_number);
        solve(&mut sc);
    }
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
