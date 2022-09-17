#![allow(unused_imports)]
use std::any::type_name;
use std::cmp::*;
use std::collections::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::stdin;
use std::ops::Bound::*;
use std::str::FromStr;

struct Scanner {
    tokens: VecDeque<String>,
    delimiters: Option<HashSet<char>>,
}
impl Scanner {
    #[allow(dead_code)]
    fn new() -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
            delimiters: None,
        }
    }

    #[allow(dead_code)]
    fn with_delimiters(delimiters: &[char]) -> Scanner {
        Scanner {
            tokens: VecDeque::new(),
            delimiters: Some(delimiters.iter().copied().collect()),
        }
    }

    #[allow(dead_code)]
    fn next<T: FromStr>(&mut self) -> T {
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

    fn receive_input(&mut self) {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read.");
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

    #[allow(dead_code)]
    fn next_line(&mut self) -> String {
        if !self.tokens.is_empty() {
            panic!("You have unprocessed token");
        }
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read.");
        line
    }
}

type Count = usize;

#[derive(Clone)]
struct MultiSet<E: Eq + Hash> {
    hash_map: HashMap<E, Count>,
}
impl<E: Eq + Hash> MultiSet<E> {
    #[allow(dead_code)]
    fn new() -> MultiSet<E> {
        MultiSet {
            hash_map: HashMap::<E, Count>::new(),
        }
    }

    #[allow(dead_code)]
    fn count(&self, e: &E) -> &Count {
        return self.hash_map.get(e).unwrap_or(&0);
    }

    #[allow(dead_code)]
    fn insert(&mut self, e: E) {
        let next = self.count(&e) + 1;
        self.hash_map.insert(e, next);
    }
}

trait BinarySearchable<T> {
    fn binary_search_leq(&self, x: &T) -> usize;
    fn binary_search_geq(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearchable<T> for [T] {
    fn binary_search_leq(&self, x: &T) -> usize {
        self.binary_search_by(|p| {
            let r = p.cmp(x);
            if r == Ordering::Greater {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .err()
        .unwrap()
    }

    fn binary_search_geq(&self, x: &T) -> usize {
        self.binary_search_by(|p| {
            let r = p.cmp(x);
            if r == Ordering::Less {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .err()
        .unwrap()
    }
}

#[allow(dead_code)]
type I = i128;
#[allow(dead_code)]
type U = usize;

fn main() {
    let mut sc = Scanner::new();
    let x1 = sc.next::<I>();
    let y1 = sc.next::<I>();
    let x2 = sc.next::<I>();
    let y2 = sc.next::<I>();
    let x3 = sc.next::<I>();
    let y3 = sc.next::<I>();
    let x4 = sc.next::<I>();
    let y4 = sc.next::<I>();
    let x5 = sc.next::<I>();
    let y5 = sc.next::<I>();
    let x6 = sc.next::<I>();
    let y6 = sc.next::<I>();
    let blacks = vec![(x3, y3, x4, y4), (x5, y5, x6, y6)];

    // one of the black entirely cover the white
    for &(x1b, y1b, x2b, y2b) in &blacks {
        if x1 >= x1b && y1 >= y1b && x2 <= x2b && y2 <= y2b {
            println!("NO");
            return;
        }
    }

    // one of the corner isn't covered
    let points = vec![(x1, y1), (x2, y1), (x1, y2), (x2, y2)];
    for &(x, y) in &points {
        let mut covered = false;
        for &(x1b, y1b, x2b, y2b) in &blacks {
            if x1b <= x && y1b <= y && x2b >= x && y2b >= y {
                covered = true;
            }
        }
        if !covered {
            println!("YES");
            return;
        }
    }

    // At this point the two black square must both cover 2 of the white square's corner.
    // If they touch, then the white square is covered, otherwise the white square isn't.
    for (i, &(x1, y1, x2, y2)) in blacks.iter().enumerate() {
        let other_i = (i + 1) % 2;
        let other_points = &blacks[other_i];
        let other_points = vec![
            (other_points.0, other_points.1),
            (other_points.2, other_points.3),
            (other_points.0, other_points.3),
            (other_points.2, other_points.1),
        ];
        for &(x, y) in &other_points {
            // if x, y is in the other square then it covers
            if x >= x1 && y >= y1 && x <= x2 && y <= y2 {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
}
