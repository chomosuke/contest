use std::io::stdin;
use std::str::FromStr;
use std::fmt::Debug;
use std::any::type_name;
use std::collections::VecDeque;

struct Scanner {
    tokens: VecDeque<String>,
}

impl Scanner {
    #[allow(dead_code)]
    fn new() -> Scanner {
        return Scanner {
            tokens: VecDeque::new(),
        };
    }

    #[allow(dead_code)]
    fn next<T: FromStr>(&mut self) -> T where
        <T as FromStr>::Err: Debug {
        let token = loop {
            let front = self.tokens.pop_front();
            if let Some(token) = front {
                break token;
            }
            self.receive_input();
        };
        return token.parse::<T>()
            .expect(&format!("input isn't a {}", type_name::<T>()));
    }

    fn receive_input(&mut self) {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Failed to read.");
        for token in  buffer.split_whitespace() {
            self.tokens.push_back(String::from(token));
        }
    }

    #[allow(dead_code)]
    fn next_line(&mut self) -> String {
        if !self.tokens.is_empty() {
            panic!("You have unprocessed token");
        }
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Failed to read.");
        return buffer;
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next::<usize>();
    let mut events = Vec::<(i64, i64)>::with_capacity(n);
    for _ in 0..n {
        events.push((sc.next::<i64>(), sc.next::<i64>()));
    }
    events.sort_unstable();
    let mut max_end = events[0].1;
    let mut count = 0;
    for (_, end) in &events[1..] {
        if end < &max_end {
            count += 1;
        } else {
            max_end = *end;
        }
    }
    println!("{}", count);
}
