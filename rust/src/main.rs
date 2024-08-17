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
        Deref, RangeBounds,
    },
    usize,
};

type I = i128;
type U = u128;

fn apply_n<E: Clone>(x: &E, n: usize, f: &impl Fn(&E, &E) -> E) -> E {
    if n == 0 {
        panic!("This function does not have an Id element.");
    } else if n == 1 {
        x.clone()
    } else if n % 2 == 0 {
        if n / 2 == 1 {
            f(x, x)
        } else {
            let x = apply_n(x, n / 2, f);
            f(&x, &x)
        }
    } else {
        f(&apply_n(x, n - 1, f), x)
    }
}

fn l2(cost_count: &Vec<U>, cost_count2: &Vec<U>, modulo: U) -> Vec<U> {
    let m = cost_count.len();
    assert_eq!(cost_count.len(), cost_count2.len());
    let mut new_cost_count = vec![0; m];
    for i in 0..m {
        for j in 0..m {
            new_cost_count[i] += cost_count[j] * cost_count2[(m - j + i) % m];
            new_cost_count[i] %= modulo;
        }
    }
    new_cost_count
}

fn solve(sc: &mut Scanner<Stdin>, pt: &mut Printer<Stdout>) {
    let n = sc.next::<usize>();
    let l = sc.next::<usize>();
    let m = sc.next::<usize>();
    let in_cost = sc.next_n::<usize>(n);
    let bet_cost = sc.next_n::<usize>(n);
    let out_cost = sc.next_n::<usize>(n);

    let modulo = 10_u128.pow(9) + 7;

    let mut in_cost_count = vec![0; m];
    for &i in &in_cost {
        let i = i % m;
        in_cost_count[i] += 1;
        in_cost_count[i] %= modulo;
    }
    let mut bet_cost_count = vec![0; m];
    for &i in &bet_cost {
        let i = i % m;
        bet_cost_count[i] += 1;
        bet_cost_count[i] %= modulo;
    }

    let bet_cost_count = if l == 2 {
        let mut b = vec![0; m];
        b[0] = 1;
        b
    } else {
        apply_n(&bet_cost_count, l - 2, &|a, b| l2(a, b, modulo))
    };

    let cost_count = l2(&in_cost_count, &bet_cost_count, modulo);

    let mut out_cost_count = vec![0; m];
    for (&i, &i2) in out_cost.iter().zip(bet_cost.iter()) {
        let i = (i + i2) % m;
        out_cost_count[i] += 1;
        out_cost_count[i] %= modulo;
    }

    let cost_count = l2(&out_cost_count, &cost_count, modulo);
    pt.println(cost_count[0]);
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    // let test_cases = sc.next::<usize>();
    // 'test: for _ in 0..test_cases {
    solve(&mut sc, &mut pt);
    // }
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
