use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

struct Customer {
    id: i64,
    num_items: i32,
    processed_items: i32,
}

struct Line {
    customers: VecDeque<Option<Customer>>,
    poped_front: usize,
}

// #[derive(Hash)]
struct LinePos {
    line_number: i64,
    line_pos: usize,
}

pub struct SupermarketCheckout {
    lines: HashMap<i64, Line>,
    customers_line_pos: HashMap<i64, LinePos>,
}

fn on_customer_exit(customer_id: i64) {
    // Don't change this implementation.
    println!("{}", customer_id);
}

impl SupermarketCheckout {
    pub fn new() -> Self {
        Self {
            lines: HashMap::new(),
            customers_line_pos: HashMap::new(),
        }
    }

    pub fn on_customer_enter(&mut self, customer_id: i64, line_number: i64, num_items: i32) {
        if !self.lines.contains_key(&line_number) {
            self.lines.insert(
                line_number,
                Line {
                    customers: VecDeque::new(),
                    poped_front: 0,
                },
            );
        }
        self.customers_line_pos.insert(
            customer_id,
            LinePos {
                line_number,
                line_pos: self.lines[&line_number].customers.len(),
            },
        );
        self.lines
            .get_mut(&line_number)
            .unwrap()
            .customers
            .push_back(Some(Customer {
                id: customer_id,
                num_items,
                processed_items: 0,
            }));
    }

    pub fn on_basket_change(&mut self, customer_id: i64, new_num_items: i32) {
        let LinePos {
            line_number,
            line_pos,
        } = self.customers_line_pos.get_mut(&customer_id).unwrap();
        let line = self.lines.get_mut(line_number).unwrap();
        let pos = *line_pos - line.poped_front;
        let customer_opt = &mut line.customers[pos];
        let customer = customer_opt.as_mut().unwrap();
        if new_num_items > customer.num_items + customer.processed_items {
            let mut customer = customer_opt.take().unwrap();
            customer.num_items = new_num_items - customer.processed_items;
            *line_pos = line.poped_front + line.customers.len();
            line.customers.push_back(Some(customer));
        } else {
            customer.num_items = new_num_items - customer.processed_items;
            if new_num_items == 0 {
                customer_opt.take();
                self.customers_line_pos.remove(&customer_id);
                on_customer_exit(customer_id);
            }
        }
    }

    pub fn on_line_service(&mut self, line_number: i64, mut num_processed_items: i32) {
        let line = self.lines.get_mut(&line_number).unwrap();
        while num_processed_items > 0 && !line.customers.is_empty() {
            let customer = if let Some(customer) = line.customers.front_mut().unwrap() {
                customer
            } else {
                line.customers.pop_front();
                continue;
            };
            let item_num = min(num_processed_items, customer.num_items);
            num_processed_items -= item_num;
            customer.num_items -= item_num;
            customer.processed_items += item_num;
            if customer.num_items == 0 {
                let customer_id = customer.id;
                line.customers.pop_front();
                self.customers_line_pos.remove(&customer_id);
                on_customer_exit(customer_id);
            }
        }
    }

    pub fn on_lines_service(&mut self) {
        let mut line_numbers: Vec<i64> = self.lines.keys().cloned().collect();
        line_numbers.sort();
        for line_number in line_numbers {
            self.on_line_service(line_number, 1);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut checkout_tracker: SupermarketCheckout = SupermarketCheckout::new();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    for _ in 0..n {
        let line = stdin_iterator.next().unwrap().unwrap();
        let parameters: Vec<&str> = line.split_whitespace().collect();

        match parameters[0].trim() {
            "CustomerEnter" => {
                let customer_id = parameters[1].parse::<i64>().unwrap();
                let line_number = parameters[2].parse::<i64>().unwrap();
                let num_items = parameters[3].parse::<i32>().unwrap();
                checkout_tracker.on_customer_enter(customer_id, line_number, num_items);
            }
            "BasketChange" => {
                let customer_id = parameters[1].parse::<i64>().unwrap();
                let new_num_items = parameters[2].parse::<i32>().unwrap();
                checkout_tracker.on_basket_change(customer_id, new_num_items);
            }
            "LineService" => {
                let line_number = parameters[1].parse::<i64>().unwrap();
                let num_processed_items = parameters[2].parse::<i32>().unwrap();
                checkout_tracker.on_line_service(line_number, num_processed_items);
            }
            "LinesService" => {
                checkout_tracker.on_lines_service();
            }
            _ => {
                panic!("Malformed input!");
            }
        };
    }
}
