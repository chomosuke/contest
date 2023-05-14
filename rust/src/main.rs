#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{BufReader, stdin},
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let l = sc.next::<u128>();
    let r = sc.next::<u128>();
    let mut resl = 0;
    let mut resr = 0;
    for i in (0..64).rev() {
        let bit: u128 = 1 << i;
        let l1 = (resl | bit) & !(bit - 1);
        let r1 = (resr | bit) & !(bit - 1);
        let l0 = (resl & !bit) | (bit - 1);
        let r0 = (resr & !bit) | (bit - 1);
        if r1 <= r && l0 >= l {
            // resr can set to 1 and resl can set to 0
            resr = resr | bit;
            resl = resl & !bit;
        } else if l1 <= r && r0 >= l {
            // resr can set to 0 and resl can set to 1
            resr = resr & !bit;
            resl = resl | bit;
        } else {
            // they have to be the same
            if r1 > r {
                // has to both be 0
                resr = resr & !bit;
                resl = resl & !bit;
            } else if l0 < l {
                // has to both be 1
                resr = resr | bit;
                resl = resl | bit;
            } else {
                panic!();
            }
        }
    }
    println!("{}", resl ^ resr);
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
