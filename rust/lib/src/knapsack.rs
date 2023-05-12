use std::cmp::max;
use std::collections::HashMap;

type Value = u64;
type Weight = u64;

pub fn knapsack_01(items: &[(Value, Weight)], max_w: u64) -> Value {
    _knapsack_01(items, 0, max_w, &mut HashMap::new())
}

fn _knapsack_01(
    items: &[(Value, Weight)],
    processed: usize,
    rem_w: Weight,
    mem: &mut HashMap<(usize, Weight), Value>,
) -> Value {
    if processed >= items.len() {
        return 0;
    }
    let key = (processed, rem_w);
    if mem.get(&key).is_none() {
        let item = items[processed];
        let max_v = if rem_w >= item.1 {
            max(
                _knapsack_01(items, processed + 1, rem_w, mem),
                _knapsack_01(items, processed + 1, rem_w - item.1, mem) + item.0,
            )
        } else {
            _knapsack_01(items, processed + 1, rem_w, mem)
        };
        mem.insert(key, max_v);
    }
    mem[&key]
}

#[test]
fn test_knapsack_01() {
    let r = knapsack_01(&[(10, 2), (20, 4), (30, 6)], 8);
    assert_eq!(40, r);
}