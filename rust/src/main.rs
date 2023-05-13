#![allow(unused_imports, dead_code)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::BufReader,
};

fn dist((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    ((x1 as i128 - x2 as i128).abs() + (y1 as i128 - y2 as i128).abs()) as u64
}

fn main() {
    let mut sc = Scanner::new(fs::File::open("input.txt").unwrap());
    let n = sc.next::<u64>();
    let m = sc.next::<u64>();
    let k = sc.next::<usize>();
    let mut bs = Vec::with_capacity(k);
    for _ in 0..k {
        let x = sc.next::<u64>();
        let y = sc.next::<u64>();
        bs.push((x, y));
    }
    let mut max_dist = 0;
    let mut max_dist_xy = bs[0];
    for x in 1..=n {
        for y in 1..=m {
            let mut min_dist = m + n;
            for b in &bs {
                let dist = dist(*b, (x, y));
                if dist < min_dist {
                    min_dist = dist;
                }
            }
            if min_dist > max_dist {
                max_dist_xy = (x, y);
                max_dist = min_dist;
            }
        }
    }
    fs::write("output.txt", format!("{} {}", max_dist_xy.0, max_dist_xy.1)).unwrap();
}

mod scanner {
    use std::collections::{HashSet, VecDeque};
    use std::io::{BufReader, Read};
    use std::{any::type_name, io::BufRead, str::FromStr};

    pub struct Scanner<R: Read> {
        tokens: VecDeque<String>,
        delimiters: Option<HashSet<char>>,
        source: BufReader<R>,
    }
    impl<R: Read> Scanner<R> {
        pub fn new(source: R) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: None,
                source: BufReader::new(source),
            }
        }

        pub fn with_delimiters(source: R, delimiters: &[char]) -> Self {
            Self {
                tokens: VecDeque::new(),
                delimiters: Some(delimiters.iter().copied().collect()),
                source: BufReader::new(source),
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
            self.source.read_line(&mut line).expect("Failed to read.");
            line.pop();
            line
        }

        fn receive_input(&mut self) {
            let mut line = String::new();
            self.source.read_line(&mut line).expect("Failed to read.");
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
