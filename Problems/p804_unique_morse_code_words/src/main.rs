use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        const MAPS: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut unique_morse: HashSet<String> = HashSet::new();

        for word in words.iter() {
            let mut morse = String::new();
            for c in word.chars() {
                morse.push_str(MAPS[c as usize - 'a' as usize]);
            }
            unique_morse.insert(morse);
        }
        unique_morse.len() as i32
    }
}

fn main() {
    assert_eq!(
        Solution::unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string()
        ]),
        2
    );
}
