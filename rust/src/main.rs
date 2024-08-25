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
type U = u64;

fn solve(sc: &mut Scanner<Stdin>, pt: &mut Printer<Stdout>) {
    let n = sc.next::<usize>();
    let m = sc.next::<U>();
    let mut arrs = Vec::with_capacity(n);
    for _ in 0..n {
        let l = sc.next::<usize>();
        arrs.push(sc.next_n::<U>(l));
    }
    let missings = arrs
        .iter()
        .map(|arr| {
            let arr: HashSet<U> = HashSet::from_iter(arr.iter().cloned());
            let mut missing = Vec::new();
            let mut i = 0;
            while missing.len() < 2 {
                if !arr.contains(&i) {
                    missing.push(i);
                }
                i += 1;
            }
            [missing[0], missing[1]]
        })
        .collect::<Vec<_>>();

    // Get duplicated_fst
    let mut fst_map = HashMap::<U, usize>::new();
    for &[m1, _m2] in &missings {
        *fst_map.entry(m1).or_default() += 1;
    }
    let duplicated_fst = fst_map
        .into_iter()
        .filter_map(|(m1, count)| if count >= 2 { Some(m1) } else { None })
        .collect::<HashSet<_>>();

    // Get graphs
    let mut graph = HashMap::<U, Vec<U>>::new();
    let mut graph_rev = HashMap::<U, Vec<U>>::new();
    for &[m1, m2] in &missings {
        graph.entry(m1).or_default().push(m2);
        graph_rev.entry(m2).or_default().push(m1);
    }

    // Get largest_reachable
    let mut reachable = HashSet::new();
    let mut to_visit = duplicated_fst.into_iter().collect::<VecDeque<_>>();
    while let Some(n) = to_visit.pop_front() {
        if !reachable.contains(&n) {
            reachable.insert(n);
            if let Some(children) = graph.get(&n) {
                to_visit.extend(children);
            }
        }
    }
    reachable.extend(missings.iter().map(|&[f, _]| f));
    let &largest_reachable = reachable.iter().max().unwrap();

    let mut snd_missings = missings
        .iter()
        .map(|&[_m, m2]| m2)
        .filter(|&m2| m2 > largest_reachable)
        .collect::<Vec<_>>();

    snd_missings.sort();

    let mut to_snd_missing = Vec::new();
    let mut all_to_snd = HashSet::new();
    for &m in snd_missings.iter().rev() {
        let mut to_snd = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back(m);
        while let Some(n) = to_visit.pop_front() {
            if !all_to_snd.contains(&n) {
                all_to_snd.insert(n);
                to_snd.insert(n);
                if let Some(children) = graph_rev.get(&n) {
                    to_visit.extend(children);
                }
            }
        }
        to_snd_missing.push((m, to_snd));
    }

    let mut ans = (m + 1) * largest_reachable;
    if m > largest_reachable {
        let n = m - largest_reachable;
        ans += n * (n + 1) / 2;
    }

    for (higher, to_m) in to_snd_missing {
        for t in to_m {
            if t <= m {
                if largest_reachable > t {
                    ans += higher - largest_reachable;
                } else {
                    ans += higher - t;
                }
            }
        }
    }
    pt.println(ans);
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
