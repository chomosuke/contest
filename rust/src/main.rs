#![allow(unused_imports, dead_code, clippy::needless_range_loop, unused_labels)]
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader},
    iter,
    mem::{self, swap},
    usize,
};

enum Action {
    Plus(u32),
    Minus(Vec<u32>),
}

fn main() {
    let mut sc = Scanner::new(stdin());
    // let mut pt = Printer::new(stdout());
    let q = sc.next::<usize>();
    let mut count_map = HashMap::<u32, usize>::new();
    let mut actions = Vec::new();
    let mut arr = Vec::new();
    let mut count = 0;
    for _ in 0..q {
        let a = sc.next::<char>();
        match a {
            '+' => {
                let n = sc.next();
                if count_map.get(&n).is_none() {
                    count += 1;
                }
                *count_map.entry(n).or_default() += 1;
                actions.push(Action::Plus(n));
                arr.push(n);
            }
            '-' => {
                let n = sc.next::<usize>();
                let d = arr.len() - n;

                let removed = arr[d..].to_vec();
                arr.truncate(d);
                for r in &removed {
                    *count_map.get_mut(r).unwrap() -= 1;
                    if count_map[r] == 0 {
                        count_map.remove(r);
                        count -= 1;
                    }
                }
                actions.push(Action::Minus(removed));
            }
            '?' => {
                println!("{}", count);
            }
            '!' => {
                let a = actions.pop().unwrap();
                match a {
                    Action::Plus(n) => {
                        *count_map.get_mut(&n).unwrap() -= 1;
                        if count_map[&n] == 0 {
                            count_map.remove(&n);
                            count -= 1;
                        }
                        arr.pop();
                    }
                    Action::Minus(v) => {
                        for n in v {
                            if count_map.get(&n).is_none() {
                                count += 1;
                            }
                            *count_map.entry(n).or_default() += 1;
                            arr.push(n);
                        }
                    }
                }
            }
            _ => panic!("{a}"),
        }
    }
}

#[allow(unused_imports)]
use io::*;
mod io {
    use std::collections::{HashSet, VecDeque};
    use std::fmt::Display;
    use std::io::{BufReader, BufWriter, Lines, Read, Write};
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

    pub struct Printer<W: Write> {
        writer: BufWriter<W>,
    }
    impl<W: Write> Printer<W> {
        pub fn new(destination: W) -> Self {
            Self {
                writer: BufWriter::new(destination),
            }
        }

        pub fn print(&mut self, s: impl Display) {
            self.writer
                .write_all(s.to_string().as_bytes())
                .expect("print failed.");
        }

        pub fn print_bytes(&mut self, b: &[u8]) {
            self.writer.write_all(b).expect("print_bytes failed.");
        }

        pub fn println(&mut self, s: impl Display) {
            self.print(s);
            self.newline();
        }

        pub fn newline(&mut self) {
            self.print_bytes(&[b'\n']);
        }

        pub fn print_iter(&mut self, mut iter: impl Iterator<Item = impl Display>) {
            if let Some(e) = iter.next() {
                self.print(&e);
                for e in iter {
                    self.print_bytes(&[b' ']);
                    self.print(&e);
                }
            }
            self.newline();
        }
    }
    impl<W: Write> Drop for Printer<W> {
        fn drop(&mut self) {
            self.writer
                .flush()
                .expect("flush failed when dropping Printer.");
        }
    }
}
