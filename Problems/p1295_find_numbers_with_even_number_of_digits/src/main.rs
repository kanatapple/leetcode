struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&num| num.to_string().len() % 2 == 0)
            .count() as i32
    }
}

fn main() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
}
