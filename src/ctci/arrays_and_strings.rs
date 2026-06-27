use std::collections::HashMap;
use std::collections::HashSet;

/* =============================== Arrays and strings ===============================
Assumptions:
All strings are assumed to use unicode characters.
Unicode scalar values are compared without normalization.
Visually identical strings may not be treated as equal and unicode normalization is
left to the user.
*/

/* Function that returns true if a string has all unique characters and false if not. If an empty string
is given, the function returns true.
Time: O(N), Space: O(N)
*/
pub fn is_unique(string: &str) -> bool {
    let mut set = HashSet::new();
    string.chars().all(|char| set.insert(char))
}

/* Function that returns true if string s1 is permutation of s2 and false if not.
If both strings are empty, the function returns true.
Time: O(N), Space: O(N)
*/
pub fn check_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut counts = HashMap::with_capacity(s1.len());
    for c in s1.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for c in s2.chars() {
        match counts.get_mut(&c) {
            Some(v) if *v > 0 => *v -= 1,
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        let test_cases = [
            ("", true),
            ("hello", false),
            ("world", true),
            ("s4fad", true),
            ("hb 627jh=j ()", false),
            ("aA", true),
            ("🚀💡", true),
            ("🚀🚀", false),
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                is_unique(input),
                expected,
                "Test failed for input string: '{}'",
                input
            );
        }
    }

    #[test]
    fn test_check_permutation() {
        let test_cases = [
            ("", "", true),
            ("abc", "bca", true),
            ("abc", "abcd", false),
            ("abc", "abd", false),
            ("aabb", "bbaa", true),
            ("hello", "olleh", true),
            ("hello", "ollhe!", false),
            ("aA", "Aa", true),
            ("🚀💡", "💡🚀", true),
            ("🚀🚀", "🚀", false),
        ];

        for (s1, s2, expected) in test_cases {
            assert_eq!(
                check_permutation(s1, s2),
                expected,
                "Failed for ('{}', '{}')",
                s1,
                s2
            );
        }
    }
}
