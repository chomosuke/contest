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

type N = u32;

fn get_nth_bit(a: N, n: usize) -> bool {
    (a >> n) % 2 == 1
}

fn set_nth_bit(a: &mut N, n: usize, b: bool) {
    if b {
        *a = *a | (1 << n)
    } else {
        *a = *a & !(1 << n)
    }
}

fn main() {
    let mut sc = Scanner::new(stdin());
    // let mut pt = Printer::new(stdout());
    let n = sc.next::<usize>();
    let k = sc.next::<usize>();
    println!("and 1 2");
    let and12 = sc.next::<u32>();
    println!("or 1 2");
    let or12 = sc.next::<u32>();
    println!("and 1 3");
    let and13 = sc.next::<u32>();
    println!("or 1 3");
    let or13 = sc.next::<u32>();
    println!("and 2 3");
    let and23 = sc.next::<u32>();
    println!("or 2 3");
    let or23 = sc.next::<u32>();
    let mut v = vec![0; n];
    for i in 0..32 {
        let and12 = get_nth_bit(and12, i);
        let or12 = get_nth_bit(or12, i);
        let and13 = get_nth_bit(and13, i);
        let or13 = get_nth_bit(or13, i);
        let and23 = get_nth_bit(and23, i);
        let or23 = get_nth_bit(or23, i);
        if and12 == or12 {
            set_nth_bit(&mut v[0], i, and12);
            set_nth_bit(&mut v[1], i, and12);
            set_nth_bit(&mut v[2], i, (and23 != or23) != and12);
        } else if and13 == or13 {
            set_nth_bit(&mut v[0], i, and13);
            set_nth_bit(&mut v[2], i, and13);
            set_nth_bit(&mut v[1], i, !and13);
        } else {
            set_nth_bit(&mut v[1], i, and23);
            set_nth_bit(&mut v[2], i, and23);
            set_nth_bit(&mut v[0], i, !and23);
        }
    }
    for i in 3..n {
        let v1 = v[0];
        println!("and 1 {}", i + 1);
        let and1i = sc.next::<u32>();
        println!("or 1 {}", i + 1);
        let or1i = sc.next::<u32>();
        for j in 0..32 {
            let and1i = get_nth_bit(and1i, j);
            let or1i = get_nth_bit(or1i, j);
            let v1 = get_nth_bit(v1, j);
            set_nth_bit(&mut v[i], j, (and1i != or1i) != v1);
        }
    }
    v.sort_unstable();
    println!("finish {}", v[k - 1])
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
