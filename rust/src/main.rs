#![allow(unused_imports, dead_code, clippy::needless_range_loop, unused_labels)]
use core::hash::Hash;
use io::*;
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader},
    iter,
    mem::{self, swap},
    ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, RangeBounds,
    },
    usize,
};

fn get_med(arr: &Vec<u64>, i_to_remove: usize) -> u64 {
    let mid = (arr.len() - 1) / 2;
    if i_to_remove > mid {
        arr[(arr.len() - 2) / 2]
    } else if i_to_remove < mid {
        arr[(arr.len() - 2) / 2 + 1]
    } else {
        if arr.len() % 2 == 0 {
            arr[mid + 1]
        } else {
            arr[mid - 1]
        }
    }
}

type U = u64;

fn search(start: U, step: U, p: impl Fn(U) -> bool) -> U {
    assert!(p(start));
    let mut index = start;
    let mut step = step;
    while step > 0 {
        if p(index + step) {
            index += step;
        } else {
            step /= 2;
        }
    }
    index + 1
}

/// The biggest is already taken out.
fn med_reachable(arr0: &Vec<u64>, arr1: &Vec<u64>, k: u64) -> u64 {
    let tot_smaller = (arr0.len() + arr1.len() - 1) / 2;
    search(1, 500_000_000, |med| {
        let smaller = arr0.partition_point(|&a| a < med);
        if smaller > tot_smaller {
            return false;
        }
        let smaller = tot_smaller - smaller;
        let mut k_needed = 0;
        for i in smaller..arr1.len() {
            // arr1 must be equal to med
            if arr1[i] < med {
                k_needed += med - arr1[i];
            } else {
                break;
            }
        }
        return k_needed <= k;
    }) - 1
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_cases = sc.next::<usize>();
    'test: for _ in 0..test_cases {
        let n = sc.next::<usize>();
        let k = sc.next::<u64>();
        let arr = sc.next_n::<u64>(n);
        let brr = sc
            .next_n::<u8>(n)
            .into_iter()
            .map(|b| b == 1)
            .collect::<Vec<_>>();
        let mut abrr = arr.into_iter().zip(brr.into_iter()).collect::<Vec<_>>();
        abrr.sort();
        let arr = abrr.iter().map(|&(a, _b)| a).collect::<Vec<_>>();
        let brr = abrr.iter().map(|&(_a, b)| b).collect::<Vec<_>>();

        let mut max = arr[arr.len() - 1] + get_med(&arr, arr.len() - 1);
        for i in 0..n {
            if brr[i] {
                if arr[i] + k >= arr[arr.len() - 1] {
                    max = max.max(arr[i] + k + get_med(&arr, i));
                }
            }
        }
        let (m, _) = abrr.pop().unwrap();
        let arr0 = abrr
            .iter()
            .filter_map(|&(a, b)| if !b { Some(a) } else { None })
            .collect::<Vec<_>>();
        let arr1 = abrr
            .iter()
            .filter_map(|&(a, b)| if b { Some(a) } else { None })
            .collect::<Vec<_>>();

        max = max.max(med_reachable(&arr0, &arr1, k) + m);
        pt.println(max);
    }
}

mod io {
    use std::collections::{HashSet, VecDeque};
    use std::fmt::Display;
    use std::io::{BufReader, BufWriter, Lines, Read, Write};
    use std::marker::PhantomData;
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

        pub fn next_n<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(self.next());
            }
            v
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
