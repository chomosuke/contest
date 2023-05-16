#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{BufReader, stdin},
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let test_cases = sc.next::<usize>();
    'outer: for _ in 0..test_cases {
        let _n = sc.next::<usize>();
        let a = sc.next_line().into_bytes();
        let b = sc.next_line().into_bytes();
        let mut counts = vec![vec![false; 20]; 20];
        for (a, b) in a.into_iter().zip(b.into_iter()) {
            let a = a - b'a';
            let b = b - b'a';
            if a > b {
                println!("-1");
                continue 'outer;
            }
            if a < b {
                counts[a as usize][b as usize] = true;
            }
        }
        let mut moves = 0;
        for a in 0..20 {
            let min_b = counts[a].iter().position(|&t| t);
            if let Some(min_b) = min_b {
                for b in (min_b + 1)..20 {
                    counts[min_b][b] = counts[min_b][b] || counts[a][b];
                }
                moves += 1;
            }
        }
        println!("{moves}");
    }
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
