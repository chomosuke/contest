use std::cmp;
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    if intervals.len() == 0 {
        intervals.push(new_interval);
        return intervals;
    }
    let mut ins: usize = 0;
    while ins < intervals.len() && new_interval[0] > intervals[ins][0] {
        // find the interval
        ins += 1;
    }
    if ins == intervals.len() {
        if new_interval[0] <= intervals[ins - 1][1] {
            intervals[ins - 1][1] = cmp::max(intervals[ins - 1][1], new_interval[1]);
        } else {
            intervals.push(new_interval);
        }
        return intervals;
    }

    // now new_interval[0] <= intervals[ins][0] and ins < intervals.length()

    let mut start: usize;

    let mut end: usize;
    if ins > 0 && intervals[ins - 1][1] >= new_interval[0] { // new interval merge with start
        start = ins - 1;
        intervals[ins - 1][1] = cmp::max(intervals[ins - 1][1], new_interval[1]);
        end = start; // end of the merge intervals[start][1] will be intervals[end][1]
    }
    else {
        start = ins;
        if intervals[ins][0] > new_interval[1] {
            intervals.insert(ins, new_interval);
            return intervals;
        } else {
            end = ins; // end of the merge intervals[start][1] will be intervals[end][1]
            intervals[end][1] = cmp::max(intervals[end][1], new_interval[1]);
            intervals[start][0] = new_interval[0];
        }
    }

    while end + 1 < intervals.len() && intervals[end][1] >= intervals[end + 1][0] {
        intervals[end + 1][1] = cmp::max(intervals[end][1], intervals[end + 1][1]);
        end += 1;
    }
    intervals[start][1] = intervals[end][1];
    remove::<Vec<i32>>(&mut intervals, start + 1, end - start);
    return intervals;
}

fn remove<T>(vec:&mut  Vec<T>, start: usize, length: usize) {
    if length == 0 {
        return;
    }
    for i in start..(vec.len() - length) {
        vec.swap(i, i + length);
    }
    vec.truncate(vec.len() - length);
}