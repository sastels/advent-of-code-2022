pub fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort();
    let mut merged: Vec<(i32, i32)> = Vec::new();

    for interval in intervals.iter() {
        if merged.is_empty() || merged.last().unwrap().1 < interval.0 - 1 {
            merged.push(*interval);
        } else {
            merged.last_mut().unwrap().1 = merged.last().unwrap().1.max(interval.1);
        }
    }
    merged
}
