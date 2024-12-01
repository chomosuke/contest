#![allow(
    unused_imports,
    dead_code,
    clippy::needless_range_loop,
    unused_labels,
    clippy::ptr_arg,
    clippy::comparison_chain,
    clippy::collapsible_else_if
)]
use core::hash::Hash;
use io::*;
use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    io::{stdin, stdout, BufReader, Stdin, Stdout},
    iter,
    mem::{self, swap},
    ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, Range, RangeBounds,
    },
};

type I = i128;
type U = u128;

const YES: &str = "YA";
const NO: &str = "TIDAK";

fn get_succ(o: &Vec<usize>, i: usize) -> Option<usize> {
    if i + 1 < o.len() {
        Some(o[i + 1])
    } else {
        None
    }
}

fn get_first_order_succ() -> {

}

fn result(succ_diff_count: usize, pt: &mut Printer<Stdout>) {
    if succ_diff_count == 0 {
        pt.println(YES);
    } else {
        pt.println(NO);
    }
}

fn solve(sc: &mut Scanner<Stdin>, pt: &mut Printer<Stdout>) {
    let n = sc.next::<usize>();
    let m = sc.next::<usize>();
    let q = sc.next::<usize>();
    let arr = sc
        .next_n::<usize>(n)
        .into_iter()
        .map(|n| n - 1)
        .collect::<Vec<_>>();
    let mut brr = sc
        .next_n::<usize>(m)
        .into_iter()
        .map(|n| n - 1)
        .collect::<Vec<_>>();
    brr.extend(&arr);

    let mut first_slide_order = Vec::new();
    let mut used_b = BTreeSet::new();
    for &b in &brr {
        if !used_b.contains(&b) {
            used_b.insert(b);
            first_slide_order.push(b);
        }
    }

    let mut appearances = vec![BTreeSet::new(); n];
    for (i, &b) in brr.iter().enumerate() {
        appearances[b].insert(i);
    }

    let mut first_slide_order = BTreeMap::new();

    let mut succ_diff_count = 0_usize;
    for i in 0..arr.len() {
        if get_succ(&arr, i) != get_first_order_succ() {
            succ_diff_count += 1;
        }
    }

    for _ in 0..q {
        result(succ_diff_count, pt);
        let i = sc.next::<usize>();
        let t = sc.next::<usize>();
        let b = brr[i];
        if b == t {
            continue;
        }

        let bf_before = *appearances[b].first().unwrap();
        appearances[b].remove(&i);
        let bf_after = *appearances[b].first().unwrap();

        let tf_before = *appearances[t].first().unwrap();
        appearances[t].remove(&i);
        let tf_after = *appearances[t].first().unwrap();

        if bf_before == bf_after {
            if *appearances[t].first().unwrap() == i {
            } else {
            }
        } else {
            if *appearances[t].first().unwrap() == i {
            } else {
            }
        }

        // update brr
        brr[i] = t;
    }
    result(succ_diff_count, pt);
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_cases = sc.next::<usize>();
    'test: for _ in 0..test_cases {
        solve(&mut sc, &mut pt);
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
