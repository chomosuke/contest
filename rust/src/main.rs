#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{BufReader, stdin},
};

fn get_diff(s1: &[u8], s2: &[u8]) -> Vec<usize> {
    let mut diff = Vec::new();
    for (i, (c1, c2)) in s1.iter().zip(s2.iter()).enumerate() {
        if c1 != c2 {
            diff.push(i);
        }
    }
    diff
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let test_cases = sc.next::<usize>();
    'outer: for _ in 0..test_cases {
        let n = sc.next::<usize>();
        sc.next::<usize>();
        let mut strs = Vec::with_capacity(n);
        for _ in 0..n {
            strs.push(sc.next_line().into_bytes());
        }
        let top = strs.pop().unwrap();
        if strs.iter().any(|s| get_diff(s, &top).len() > 2) {
            println!("-1");
            continue 'outer;
        }
        let diff2 = strs.iter().filter_map(|s| {
            let diff = get_diff(s, &top);
            if diff.len() == 2 {
                Some((diff, s))
            } else {
                None
            }
        }).next();
        if diff2.is_none() {
            println!("{}", String::from_utf8(top).unwrap());
            continue 'outer;
        }
        let (d, diff2) = diff2.unwrap();
        for d in d {
            let mut cand = top.clone();
            cand[d] = diff2[d];
            if strs.iter().all(|s| get_diff(s, &cand).len() < 2) {
                println!("{}", String::from_utf8(cand).unwrap());
                continue 'outer;
            }
        }
        println!("-1");
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
