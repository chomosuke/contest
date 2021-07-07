use std::collections::VecDeque;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1: &[u8] = word1.as_bytes();
    let word2: &[u8] = word2.as_bytes();
    let mut queue: VecDeque<(i32, usize, usize)> = VecDeque::new(); // word1[..second] are first away from word2[..third]
    let mut current: (i32, usize, usize) = (0, 0, 0);
    while current.1 != word1.len() || current.2 != word2.len() {
        if current.1 == word1.len() {
            // extend current.2
            queue.push_back((current.0 + 1, current.1, current.2 + 1));
        } else if current.2 == word2.len() {
            // extend current.1
            queue.push_back((current.0 + 1, current.1 + 1, current.2));
        } // at this point both word1.get(current.1) and word2.get(current.2) will be fine
        else if word1.get(current.1) == word2.get(current.2) {
            // does match
            queue.push_front((current.0, current.1 + 1, current.2 + 1));
        } else { // doesn't match
            // change current char
            queue.push_back((current.0 + 1, current.1 + 1, current.2 + 1));
            // insert new char in one of them (equivalent to removing one in another)
            queue.push_back((current.0 + 1, current.1 + 1, current.2));
            queue.push_back((current.0 + 1, current.1, current.2 + 1));
        }
        current = queue.pop_front().unwrap();
        println!("{}, {}, {}", current.0, current.1, current.2);
    }
    return current.0;
}