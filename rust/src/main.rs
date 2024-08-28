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
    usize,
};

type I = i128;
type U = u128;

type Int = i128;

fn find_linear_comb(a: Int, b: Int) -> (Int, Int, Int) {
    let mut x_a1 = 1;
    let mut y_a1 = 0;
    let mut x_b1 = 0;
    let mut y_b1 = 1;
    let mut a1 = a;
    let mut b1 = b;
    while b1 != 0 {
        let q = a1 / b1;
        let a2 = b1;
        let b2 = a1 - q * b1;
        let x_a2 = x_b1;
        let y_a2 = y_b1;
        let x_b2 = x_a1 - q * x_b1;
        let y_b2 = y_a1 - q * y_b1;

        a1 = a2;
        b1 = b2;
        x_a1 = x_a2;
        y_a1 = y_a2;
        x_b1 = x_b2;
        y_b1 = y_b2;
    }
    (a1, x_a1, y_a1)
}

struct LDESIter {
    xy: (Int, Int),
    diff: (Int, Int),
    k_range: Range<Int>,
}
impl Iterator for LDESIter {
    type Item = (Int, Int);

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.k_range.next()?;
        Some((self.xy.0 + k * self.diff.0, self.xy.1 + k * self.diff.1))
    }
}

fn linear_diophantine_eq_sols(
    a: Int,
    b: Int,
    c: Int,
    x_range: Range<Int>,
    y_range: Range<Int>,
) -> impl Iterator<Item = (Int, Int)> {
    let (gcd, x, y) = find_linear_comb(a, b);
    if c % gcd != 0 {
        return LDESIter {
            xy: (0, 0),
            diff: (0, 0),
            k_range: 0..0,
        };
    }
    let n = c / gcd;
    let x = x * n;
    let y = y * n;

    let mut x_diff = b / gcd;
    let mut y_diff = -a / gcd;
    if x_diff < 0 {
        x_diff = -x_diff;
        y_diff = -y_diff;
    }

    let x_end = x_range.end - 1;
    let x_start = x_range.start;
    let y_end = y_range.end - 1;
    let y_start = y_range.start;

    fn div_ceil(x: Int, y: Int) -> Int {
        if (x < 0) != (y < 0) {
            x / y
        } else {
            (x - 1) / y + 1
        }
    }

    fn div_floor(x: Int, y: Int) -> Int {
        if (x < 0) != (y < 0) {
            (x + 1) / y - 1
        } else {
            x / y
        }
    }

    let mut k_start = Int::MIN;
    let mut k_end = Int::MAX;
    if x_diff == 0 {
        if x_start <= x && x < x_end {
            return LDESIter {
                xy: (x, 0),
                diff: (0, 1),
                k_range: y_start..(y_end + 1),
            };
        } else {
            return LDESIter {
                xy: (0, 0),
                diff: (0, 0),
                k_range: 0..0,
            };
        }
    }
    if x_diff < 0 {
        k_start = k_start.max(div_ceil(x_end - x, x_diff));
        k_end = k_end.min(div_floor(x_start - x, x_diff));
    } else {
        k_start = k_start.max(div_ceil(x_start - x, x_diff));
        k_end = k_end.min(div_floor(x_end - x, x_diff));
    }

    if y_diff == 0 {
        if y_start <= y && y < y_end {
            return LDESIter {
                xy: (0, y),
                diff: (1, 0),
                k_range: x_start..(x_end + 1),
            };
        } else {
            return LDESIter {
                xy: (0, 0),
                diff: (0, 0),
                k_range: 0..0,
            };
        }
    }
    if y_diff < 0 {
        k_start = k_start.max(div_ceil(y_end - y, y_diff));
        k_end = k_end.min(div_floor(y_start - y, y_diff));
    } else {
        k_start = k_start.max(div_ceil(y_start - y, y_diff));
        k_end = k_end.min(div_floor(y_end - y, y_diff));
    }

    LDESIter {
        xy: (x, y),
        diff: (x_diff, y_diff),
        k_range: k_start..(k_end + 1),
    }
}

fn solve(sc: &mut Scanner<Stdin>, pt: &mut Printer<Stdout>) {
    let x = sc.next::<I>();
    let y = sc.next::<I>();
    let range = -(10_i128.pow(18))..(10_i128.pow(18) + 1);
    if let Some((b, a)) = linear_diophantine_eq_sols(x, -y, 2, range.clone(), range.clone()).next()
    {
        pt.println(format!("{a} {b}"));
    } else if let Some((b, a)) =
        linear_diophantine_eq_sols(x, -y, -2, range.clone(), range.clone()).next()
    {
        pt.println(format!("{a} {b}"));
    } else {
        pt.println(-1);
    }
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
