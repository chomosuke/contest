
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let intervals = intervals;

    let mut result: Vec<Vec<i32>> = Vec::new();
    result.push(vec![intervals[0][0]]);
    for i in 1..intervals.len() {
        if intervals[i - 1][1] < intervals[i][0] { // not overlapping
            let len = result.len();
            result[len - 1].push(intervals[i - 1][1]); // finish the old interval
            result.push(vec![intervals[i][0]]); // start the new interval
        }
    }
    let len = result.len();
    result[len - 1].push(intervals[intervals.len() - 1][1]); // finish the last interval

    // O(nlogn) times

    return result;
}