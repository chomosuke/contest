#![allow(unused_imports)]
use std::any::type_name;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::str::FromStr;

struct Scanner {
    tokens: VecDeque<String>,
}
impl Scanner {
    #[allow(dead_code)]
    fn new() -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
        }
    }

    #[allow(dead_code)]
    fn next<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let token = loop {
            let front = self.tokens.pop_front();
            if let Some(token) = front {
                break token;
            }
            self.receive_input();
        };
        token
            .parse::<T>()
            .unwrap_or_else(|_| panic!("input isn't a {}", type_name::<T>()))
    }

    fn receive_input(&mut self) {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read.");
        for token in buffer.split_whitespace() {
            self.tokens.push_back(String::from(token));
        }
    }

    #[allow(dead_code)]
    fn next_line(&mut self) -> String {
        if !self.tokens.is_empty() {
            panic!("You have unprocessed token");
        }
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Failed to read.");
        buffer
    }
}

type Count = usize;

#[derive(Clone)]
struct CountMap<K: Eq + Hash> {
    hash_map: HashMap<K, Count>,
}
impl<K: Eq + Hash> CountMap<K> {
    #[allow(dead_code)]
    fn new() -> CountMap<K> {
        CountMap {
            hash_map: HashMap::<K, Count>::new(),
        }
    }

    #[allow(dead_code)]
    fn get(&self, key: &K) -> &Count {
        return self.hash_map.get(key).unwrap_or(&0);
    }

    #[allow(dead_code)]
    fn change(&mut self, key: K, value: Count) {
        let next = self.get(&key) + value;
        self.hash_map.insert(key, next);
    }
}

type Point = (i128, i128);

fn main() {
    let mut sc = Scanner::new();
    let t = sc.next::<i128>();

    for _ in 0..t {
        let n = sc.next::<usize>();
        let k = sc.next::<i128>();

        let mut points = Vec::<Point>::with_capacity(n);
        for _ in 0..n {
            let point = (sc.next::<i128>(), sc.next::<i128>());
            points.push(point);
        }

        let edges = find_edges(&points);

        println!("{}", if solve(&points, &edges, k) { 1 } else { -1 });
    }
}

type Edges = (Point, Point, Point, Point);

fn find_edges(points: &Vec<Point>) -> Edges {
    let mut tl = points[0];
    let mut tr = points[0];
    let mut bl = points[0];
    let mut br = points[0];
    for point in points.iter() {
        if point.0 + point.1 < tl.0 + tl.1 {
            tl = *point;
        }
        if point.0 - point.1 > tr.0 - tr.1 {
            tr = *point;
        }
        if point.0 - point.1 < bl.0 - bl.1 {
            bl = *point;
        }
        if point.0 + point.1 > br.0 + br.1 {
            br = *point;
        }
    }

    (tl, tr, bl, br)
}

fn solve(points: &Vec<Point>, edges: &Edges, k: i128) -> bool {
    for point in points.iter() {
        if md(&edges.0, point) <= k
        && md(&edges.1, point) <= k
        && md(&edges.2, point) <= k
        && md(&edges.3, point) <= k {
            return true;
        }
    }
    return false;
}

fn md(p1: &Point, p2: &Point) -> i128 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

