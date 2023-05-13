#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{BufReader, stdin},
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let aboves = sc.next_line().into_bytes().into_iter().map(|b| b == b'+').collect::<Vec<_>>();

    // construct linked list
    let mut next = Vec::new();
    let mut prev = Vec::new();
    for i in 0..aboves.len() {
        if i == 0 {
            prev.push(None);
        } else {
            prev.push(Some(i - 1));
        }
        if i == aboves.len() - 1 {
            next.push(None);
        } else {
            next.push(Some(i + 1));
        }
    }

    let mut walk = 0;
    while let Some(walk2) = next[walk] {
        if aboves[walk] == aboves[walk2] {
            if let Some(p) = prev[walk] {
                next[p] = next[walk2];
            }
            if let Some(n) = next[walk2] {
                prev[n] = prev[walk];
            }
            if let Some(p) = prev[walk] {
                walk = p;
            } else if let Some(n) = next[walk2] {
                walk = n;
            } else {
                println!("Yes");
                return;
            }
        } else {
            walk = walk2;
        }
    }
    println!("No");
}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::io::{BufReader, Read, Lines};
    use std::{any::type_name, io::BufRead, str::FromStr};

    pub struct Scanner<R: Read> {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
        lines: Lines<BufReader<R>>,
    }
    impl<R: Read> Scanner<R> {
        pub fn new(source: R) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: None,
                lines: BufReader::new(source).lines(),
            }
        }

        pub fn with_delimiters(source: R, delimiters: &[char]) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
                lines: BufReader::new(source).lines(),
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
            let line = self.lines.next().and_then(|e| e.ok()).expect("Failed to read.");
            line
        }

        fn receive_input(&mut self) {
            let line = self.lines.next().and_then(|e| e.ok()).expect("Failed to read.");
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
