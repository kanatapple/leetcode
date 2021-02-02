struct Solution {}

impl Solution {
    pub fn to_lower_case(str: String) -> String {
        str.to_lowercase()
    }
}

fn main() {
    assert_eq!(Solution::to_lower_case("LOVELY".to_string()), "lovely");
}
