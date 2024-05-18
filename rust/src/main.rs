#![allow(unused_imports, dead_code, clippy::needless_range_loop, unused_labels)]
use io::*;
use std::{
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader},
    iter,
    mem::{self, swap},
    usize,
};

fn get_diff(s0: &[u8], s1: &[u8]) -> Vec<usize> {
    s0.iter()
        .zip(s1.iter())
        .map(|(c0, c1)| c0 != c1)
        .enumerate()
        .filter_map(|(i, ne)| if ne { Some(i) } else { None })
        .collect::<Vec<usize>>()
}

fn generate(s0: &[u8], s1: &[u8], repeats: bool) -> Vec<(usize, usize)> {
    let diff = get_diff(s0, s1);
    match diff.len() {
        0 | 1 => unreachable!(),
        2 => {
            let d1 = diff[0];
            let d2 = diff[1];
            let mut pos = Vec::new(); // yes swap
            if repeats {
                pos.push((d1, d2));
                pos.push((s0.len(), s0.len())) // no swap
            }

            for i in 0..s0.len() {
                if i == d1 || i == d2 {
                    continue;
                }
                if s0[i] == s0[d1] {
                    pos.push((min(i, d2), max(i, d2)))
                }
                if s0[i] == s0[d2] {
                    pos.push((min(i, d1), max(i, d1)))
                }
            }

            pos
        }
        3 | 4 => {
            let mut pos = Vec::new();
            let s0 = diff.iter().map(|&d| s0[d]).collect::<Vec<_>>();
            let s1 = diff.iter().map(|&d| s1[d]).collect::<Vec<_>>();
            for i in 0..(diff.len() - 1) {
                for j in (i + 1)..diff.len() {
                    let mut s0 = s0.clone();
                    let temp = s0[i];
                    s0[i] = s0[j];
                    s0[j] = temp;
                    if get_diff(&s0, &s1).len() == 2 {
                        pos.push((diff[i], diff[j]));
                    }
                }
            }
            pos
        }
        _ => {
            vec![]
        }
    }
}

fn filter(s0: &[u8], s1: &[u8], pos: &mut Vec<(usize, usize)>, repeats: bool) {
    let diff = get_diff(s0, s1);

    match diff.len() {
        0 | 1 => unreachable!(),
        2 => {
            let d1 = diff[0];
            let d2 = diff[1];
            pos.retain(|&(i1, i2)| {
                if i1 == s0.len() && i2 == s0.len() {
                    // s0 no swap
                    // possible if some letters show up twice
                    repeats
                } else if d1 == i1 && d2 == i2 {
                    // s1 no swap
                    // possible if some letters show up twice
                    repeats
                } else if d1 == i1 {
                    let d = d2;
                    let i = i2;
                    s0[d] == s0[i]
                } else if d2 == i2 {
                    let d = d1;
                    let i = i1;
                    s0[d] == s0[i]
                } else if d1 == i2 {
                    let d = d2;
                    let i = i1;
                    s0[d] == s0[i]
                } else {
                    false
                }
            });
        }
        3 | 4 => {
            let s0 = diff.iter().map(|&d| s0[d]).collect::<Vec<_>>();
            let s1 = diff.iter().map(|&d| s1[d]).collect::<Vec<_>>();
            pos.retain(|&(i1, i2)| {
                let mut s0 = s0.clone();
                if let (Some(i1), Some(i2)) = (
                    diff.iter().position(|&d| d == i1),
                    diff.iter().position(|&d| d == i2),
                ) {
                    let temp = s0[i1];
                    s0[i1] = s0[i2];
                    s0[i2] = temp;
                    get_diff(&s0, &s1).len() == 2
                } else {
                    false
                }
            });
        }
        _ => {
            pos.clear();
        }
    }
    return;
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    // let test_case = sc.next::<usize>();
    // 'test: for _ in 0..test_case {
    let k = sc.next::<usize>();
    let _n = sc.next::<usize>();
    let srr = sc
        .next_n::<String>(k)
        .map(String::into_bytes)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if srr.len() == 1 {
        let mut s = srr.into_iter().next().unwrap();
        let t = s[0];
        s[0] = s[1];
        s[1] = t;
        pt.println(String::from_utf8(s).unwrap());
        return;
    }
    let bags = srr
        .iter()
        .map(|s| {
            let mut s = s.clone();
            s.sort();
            s
        })
        .collect::<Vec<_>>();
    if bags.iter().skip(1).any(|b| *b != bags[0]) {
        pt.println(-1);
        return;
    }
    let repeats = bags[0].windows(2).any(|cs| {
        let [c1, c2] = cs else { unreachable!() };
        c1 == c2
    });
    let mut pos = generate(&srr[0], &srr[1], repeats);
    for i in 2..srr.len() {
        filter(&srr[0], &srr[i], &mut pos, repeats);
    }
    if pos.len() > 0 {
        let mut s = srr.into_iter().next().unwrap();
        let p = pos[0];
        let t = s[p.0];
        s[p.0] = s[p.1];
        s[p.1] = t;
        pt.println(String::from_utf8(s).unwrap());
    } else {
        pt.println(-1);
    }
    // }
}

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
