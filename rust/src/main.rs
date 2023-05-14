#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{BufReader, stdin},
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let n = sc.next::<usize>();
    let p = sc.next::<usize>() - 1;
    let str = sc.next_line().into_bytes();
    let mut flips = Vec::new();
    let mut presses = 0;
    for i in 0..(n/2) {
        let c1 = (str[i] - b'a') as i8;
        let c2 = (str[n - i - 1] - b'a') as i8;
        if c1 != c2 {
            flips.push(i);
            presses += (c1 - c2).abs().min(c1 + 26 - c2).min(c2 + 26 - c1) as u64;
        }
    }

    if presses == 0 {
        println!("0");
        return;
    }

    let p = p.min(n - p - 1) as i128;
    let last = flips.len() - 1;
    let last = flips[last] as i128;
    let first = flips[0] as i128;
    presses += (p - last).abs().min((p - first).abs()) as u64;
    presses += (last - first) as u64;
    println!("{presses}");
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
