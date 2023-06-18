#![allow(unused_imports, dead_code, clippy::needless_range_loop, unused_labels)]
use std::{
    arch::x86_64::_mm_testz_si128,
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader},
    mem,
};

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_cases = sc.next::<u32>();
    'case: for _ in 0..test_cases {
        let a = sc.next::<u32>();
        let b = sc.next::<u32>();
        let c = sc.next::<u32>();
        let k = sc.next::<u64>();
        if c > max(a, b) + 1 || c < max(a, b) {
            pt.println("-1");
            continue 'case;
        }
        let min_a = 10_u64.pow(a - 1);
        let max_a = 10_u64.pow(a) - 1;
        let min_b = 10_u64.pow(b - 1);
        let max_b = 10_u64.pow(b) - 1;
        let min_c = 10_u64.pow(c - 1);
        let max_c = 10_u64.pow(c) - 1;
        let mut t = 1;
        // determine a
        let mut a1 = min_a.max(min_c - max_b.min(min_c));
        while t <= k {
            if a1 > max_a || a1 + min_b > max_c {
                pt.println(&-1);
                continue 'case;
            }
            let min_b = (min_c - a1.min(min_c)).max(min_b);
            let max_b = (max_c - a1).min(max_b);
            let bs = max_b - min_b + 1;
            a1 += 1;
            t += bs;
        }
        a1 -= 1;
        let min_b = (min_c - a1.min(min_c)).max(min_b);
        let max_b = (max_c - a1).min(max_b);
        let bs = max_b - min_b + 1;
        t -= bs;
        // determine b
        let mut b1 = min_b;

        b1 += k - t;
        // t += k - t;
        if b1 > max_b {
            pt.println(&-1);
            continue 'case;
        }
        pt.println(&format!("{} + {} = {}", a1, b1, a1 + b1));
    }
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

        pub fn print(&mut self, s: &(impl Display + ?Sized)) {
            self.writer
                .write_all(s.to_string().as_bytes())
                .expect("print failed.");
        }

        pub fn print_bytes(&mut self, b: &[u8]) {
            self.writer.write_all(b).expect("print_bytes failed.");
        }

        pub fn println(&mut self, s: &(impl Display + ?Sized)) {
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
#[allow(unused_imports)]
use io::*;
