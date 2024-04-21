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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct PState {
    remaining: usize,
    value: u64,
    stealing_time: u64,
    leaving_time: u64,
}

struct PPrediction {
    stealing_time: u64,
    value: u64,
    arrival_time: u64,
    leaving_time: u64,
}

const P: usize = 5;

type PStates = [PState; P];

fn max_value(
    time: u64,
    p_states: PStates,
    parking_preds: [&[PPrediction]; P],
    memo: &mut HashMap<(PStates, u64), u64>,
) -> u64 {
    let k = (p_states, time);
    if !memo.contains_key(&k) {
        let mut max_v = 0;
        // take next wheel
        for i in 0..P {
            let PState {
                remaining,
                value,
                stealing_time,
                leaving_time,
            } = p_states[i];
            if remaining > 0 && stealing_time + time <= leaving_time {
                // steal this wheel.
                let mut p_states = p_states;
                let time = time + stealing_time;
                p_states[i].remaining = remaining - 1;

                let mut i = 0;
                let parking_preds = parking_preds.map(|p_pred| {
                    let p_pred = if p_pred.len() > 0 && p_pred[0].arrival_time <= time {
                        // replace car in park
                        let PPrediction {
                            stealing_time,
                            value,
                            leaving_time,
                            ..
                        } = p_pred[0];
                        p_states[i] = PState {
                            remaining: 4,
                            value,
                            stealing_time,
                            leaving_time,
                        };
                        &p_pred[1..]
                    } else {
                        &p_pred
                    };
                    i += 1;
                    return p_pred;
                });

                let value = value + max_value(time, p_states, parking_preds, memo);
                max_v = max(value, max_v);
            }
        }
        memo.insert(k, max_v);
    }
    memo[&k]
}

fn main() {
    let mut sc = Scanner::new(stdin());
    let mut pt = Printer::new(stdout());
    let n = sc.next::<usize>();
    let mut all_parking_preds = Vec::with_capacity(n);
    for _ in 0..n {
        let arrival_time = sc.next::<u64>();
        let leaving_time = sc.next::<u64>();
        let stealing_time = sc.next::<u64>();
        let value = sc.next::<u64>();
        all_parking_preds.push(PPrediction {
            arrival_time,
            leaving_time,
            stealing_time,
            value,
        });
    }
    all_parking_preds.sort_by_key(|p| p.arrival_time);
    let mut parking_preds = [(); P].map(|_| Vec::<PPrediction>::new());
    for cur_pred in all_parking_preds {
        let mut pushed = false;
        for i in 0..P {
            if let Some(last_p) = parking_preds[i].last() {
                if last_p.leaving_time <= cur_pred.arrival_time {
                    parking_preds[i].push(cur_pred);
                    pushed = true;
                    break;
                }
            }
        }
        assert!(pushed);
    }
    let mut memo = HashMap::new();
    let max_v = max_value(
        0,
        [PState {
            remaining: 4,
            leaving_time: 0,
            stealing_time: 1,
            value: 0,
        }; P],
        parking_preds.each_ref().map(|v| &v[..]),
        &mut memo,
    );
    pt.println(max_v);
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
