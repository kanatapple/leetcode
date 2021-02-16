use std::cmp;

struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut highest = 0;
        let mut now = 0;
        for point in gain.iter() {
            now += point;
            highest = cmp::max(highest, now);
        }
        highest
    }
}

fn main() {
    assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
}
