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
    let mut arr = sc.next_n(n).collect::<Vec<u64>>();
    // start with no violation.
    // once violation is encountered, record it.
    // once second violation is encountered, this is one subsegment.
    // subsegment restarted from the first and second violation is recorded as the first.
    let mut longest = 0;
    for _ in 0..2 {
        let mut start = 0;
        let mut i = 0;
        loop {
            i += 1;
            if i >= arr.len() {
                break;
            }
            if arr[i - 1] + 1 > arr[i] {
                break;
            }
        }
        let mut violation = i;
        i += 1;
        while i < arr.len() {
            if arr[i - 1] + 1 > arr[i] {
                longest = longest.max(i - start);
                start = violation;
                violation = i;
            } else if i >= 2 && arr[i - 2] + 2 > arr[i] {
                longest = longest.max(i - start);
                start = i - 2;
            }
            i += 1;
        }
        longest = longest.max(arr.len() - start);

        arr = arr
            .into_iter()
            .rev()
            .map(|a| u64::MAX / 2 - a)
            .collect::<Vec<_>>();
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
