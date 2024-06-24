#![allow(unused_imports, dead_code, clippy::needless_range_loop, unused_labels)]
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

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_cases = sc.next::<usize>();
    'test: for _ in 0..test_cases {}
}

fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
    let mut border = BinaryHeap::<Reverse<(i32, usize, usize)>>::new();
    let mut sum = 0;
    let mut in_border = vec![vec![false; height_map[0].len()]; height_map.len()];

    fn push_border(
        i: usize,
        j: usize,
        in_border: &mut Vec<Vec<bool>>,
        border: &mut BinaryHeap<Reverse<(i32, usize, usize)>>,
        height_map: &Vec<Vec<i32>>,
    ) {
        in_border[i][j] = true;
        border.push(Reverse((height_map[i][j], i, j)));
    }

    for i in 0..height_map.len() {
        push_border(i, 0, &mut in_border, &mut border, &height_map);
        push_border(
            i,
            height_map[0].len() - 1,
            &mut in_border,
            &mut border,
            &height_map,
        );
    }
    for j in 1..(height_map[0].len() - 1) {
        push_border(0, j, &mut in_border, &mut border, &height_map);
        push_border(
            height_map.len() - 1,
            j,
            &mut in_border,
            &mut border,
            &height_map,
        );
    }

    while let Some(Reverse((height, i, j))) = border.pop() {
        let neightbor = [
            (i + 1 + 1, j + 1),
            (i + 1 - 1, j + 1),
            (i + 1, j + 1 + 1),
            (i + 1, j + 1 - 1),
        ]
        .into_iter()
        .filter_map(|(i, j)| {
            if i > 0 && j > 0 {
                Some((i - 1, j - 1))
            } else {
                None
            }
        })
        .filter(|&(i, j)| i < height_map.len() && j < height_map[0].len())
        .filter(|&(i, j)| !in_border[i][j])
        .collect::<Vec<_>>();

        for (ni, nj) in neightbor {
            if height_map[ni][nj] < height {
                sum += height - height_map[ni][nj];
                height_map[ni][nj] = height;
            }
            push_border(ni, nj, &mut in_border, &mut border, &height_map);
        }
    }

    sum
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
