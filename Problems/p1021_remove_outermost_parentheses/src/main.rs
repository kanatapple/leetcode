struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::new();
        let mut depth = 0;
        for c in s.chars() {
            if c == ')' {
                depth -= 1;
            }
            if depth != 0 {
                result.push(c);
            }
            if c == '(' {
                depth += 1;
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())".to_string()),
        "()()()"
    );
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
        "()()()()(())"
    );
    assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
}
