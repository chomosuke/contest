#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, BufReader},
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let n = sc.next();
    let arr = sc.next_n(n).collect::<Vec<u64>>();

    let mut inc_til = vec![1usize; arr.len()];
    for i in 1..arr.len() {
        if arr[i - 1] < arr[i] {
            inc_til[i] = inc_til[i - 1] + 1;
        }
    }
    let inc_til = inc_til;

    let mut inc_from = vec![1usize; arr.len()];
    for i in (0..(arr.len() - 1)).rev() {
        if arr[i] < arr[i + 1] {
            inc_from[i] = inc_from[i + 1] + 1;
        }
    }

    let mut longest = 1;
    for i in 0..arr.len() {
        if i > 0 && i + 1 < arr.len() && arr[i - 1] + 1 < arr[i + 1] {
            longest = longest.max(inc_til[i - 1] + inc_from[i + 1] + 1);
        }
        if i > 0 {
            longest = longest.max(inc_til[i - 1] + 1);
        }
        if i + 1 < arr.len() {
            longest = longest.max(inc_from[i + 1] + 1);
        }
    }

    println!("{longest}");
}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::fmt::Display;
    use std::io::{BufReader, Lines, Read};
    use std::marker::PhantomData;
    use std::{any::type_name, io::BufRead, str::FromStr};

    pub struct ScannerIter<'a, R: Read, T> {
        remaining: usize,
        sc: &'a mut Scanner<R>,
        item: PhantomData<T>,
    }

    impl<R: Read, T: FromStr> Iterator for ScannerIter<'_, R, T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.remaining == 0 {
                None
            } else {
                self.remaining -= 1;
                Some(self.sc.next::<T>())
            }
        }
    }

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

        pub fn next_n<T: FromStr>(&mut self, n: usize) -> ScannerIter<'_, R, T> {
            ScannerIter {
                remaining: n,
                sc: self,
                item: PhantomData,
            }
        }

        pub fn next_line(&mut self) -> String {
            assert!(self.tokens.is_empty(), "You have unprocessed token");
            self.lines
                .next()
                .and_then(|e| e.ok())
                .expect("Failed to read.")
        }

        fn receive_input(&mut self) {
            let line = self
                .lines
                .next()
                .and_then(|e| e.ok())
                .expect("Failed to read.");
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

    pub fn print_iter(mut iter: impl Iterator<Item = impl Display>) {
        if let Some(e) = iter.next() {
            print!("{e}");
            for e in iter {
                print!(" {e}");
            }
        }
        println!();
    }
}
#[allow(unused_imports)]
use scanner::*;
