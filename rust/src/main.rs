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

fn update(
    children: &Vec<Vec<usize>>,
    parent: &Vec<usize>,
    black: &mut Vec<bool>,
    num_child_black: &mut Vec<usize>,
    toggle: usize,
    num_groups: &mut usize,
    num_2_child: &mut usize,
    num_3_child: &mut usize,
) {
    let neighbor_count = num_child_black[toggle]
        + if black[parent[toggle]] && parent[toggle] != toggle {
            1
        } else {
            0
        };
    if black[toggle] {
        black[toggle] = false;
        if neighbor_count == 0 {
            *num_groups -= 1;
        } else {
            *num_groups += neighbor_count - 1;
        }
        if parent[toggle] != toggle {
            num_child_black[parent[toggle]] -= 1;
            if num_child_black[parent[toggle]] == 1 {
                *num_2_child -= 1;
            }
            if num_child_black[parent[toggle]] == 2 {
                *num_3_child -= 1;
            }
        }
        if num_child_black[toggle] >= 2 {
            *num_2_child -= 1;
        }
        if num_child_black[toggle] >= 3 {
            *num_3_child -= 1;
        }
    } else {
        black[toggle] = true;
        if neighbor_count == 0 {
            *num_groups += 1;
        } else {
            *num_groups -= neighbor_count - 1;
        }
        if parent[toggle] != toggle {
            num_child_black[parent[toggle]] += 1;
            if num_child_black[parent[toggle]] == 2 {
                *num_2_child += 1;
            }
            if num_child_black[parent[toggle]] == 3 {
                *num_3_child += 1;
            }
        }
        if num_child_black[toggle] >= 2 {
            *num_2_child += 1;
        }
        if num_child_black[toggle] >= 3 {
            *num_3_child += 1;
        }
    }
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let test_case = sc.next::<usize>();
    'test: for _ in 0..test_case {
        let n = sc.next::<usize>();
        let q = sc.next::<usize>();
        let colors_in = sc.next_n::<u8>(n).map(|c| c == 1).collect::<Vec<_>>();
        let mut adj_nodes = vec![Vec::<usize>::new(); n];
        for _ in 0..(n - 1) {
            let u = sc.next::<usize>() - 1;
            let v = sc.next::<usize>() - 1;
            adj_nodes[u].push(v);
            adj_nodes[v].push(u);
        }

        let mut children = vec![Vec::new(); n];
        let mut parent = vec![0; n];
        let mut to_visit = vec![(0, 0)];
        while !to_visit.is_empty() {
            let mut next_visit = Vec::new();
            for (p, v) in to_visit {
                let nvs = adj_nodes[v]
                    .iter()
                    .filter(|&&nv| nv != p)
                    .map(|&v| v)
                    .collect::<Vec<_>>();
                next_visit.extend(nvs.iter().map(|&nv| (v, nv)));
                children[v] = nvs;
                parent[v] = p;
            }
            to_visit = next_visit;
        }

        let mut num_child_black = vec![0; n];
        let mut num_groups = 0;
        let mut num_2_child = 0;
        let mut num_3_child = 0;

        let mut black = vec![false; n];
        for (i, is_black) in colors_in.into_iter().enumerate() {
            if is_black {
                update(
                    &children,
                    &parent,
                    &mut black,
                    &mut num_child_black,
                    i,
                    &mut num_groups,
                    &mut num_2_child,
                    &mut num_3_child,
                );
                // pt.print_iter([num_groups, num_2_child].iter());
            }
        }

        for _ in 0..q {
            let u = sc.next::<usize>() - 1;
            update(
                &children,
                &parent,
                &mut black,
                &mut num_child_black,
                u,
                &mut num_groups,
                &mut num_2_child,
                &mut num_3_child,
            );
            // pt.print_iter([num_groups, num_2_child].iter());
            if num_2_child <= 1 && num_3_child == 0 && num_groups == 1 {
                pt.println("Yes");
            } else {
                pt.println("No");
            }
        }
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
