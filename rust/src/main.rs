#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, BufReader},
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let test_cases = sc.next::<u32>();
    'outer: for _ in 0..test_cases {
        let n = sc.next::<usize>();
        let same = sc
            .next_line()
            .into_bytes()
            .into_iter()
            .map(|b| b == b'1')
            .collect::<Vec<_>>();
        let len_1 = same.iter().filter(|&&s| s).count();
        if !same[0] || !same[same.len() - 1] || len_1 % 2 != 0 {
            println!("NO");
            continue 'outer;
        }
        let mut a = Vec::with_capacity(n);
        let mut b = Vec::with_capacity(n);
        let mut s_count = 0;
        let mut d_count = 0;
        for same in same {
            if same {
                if s_count < len_1 / 2 {
                    a.push(b'(');
                    b.push(b'(');
                } else {
                    a.push(b')');
                    b.push(b')');
                }
                s_count += 1;
            } else {
                if d_count % 2 == 0 {
                    a.push(b'(');
                    b.push(b')');
                } else {
                    a.push(b')');
                    b.push(b'(');
                }
                d_count += 1;
            }
        }
        println!("YES");
        println!("{}", String::from_utf8(a).unwrap());
        println!("{}", String::from_utf8(b).unwrap());
    }
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
