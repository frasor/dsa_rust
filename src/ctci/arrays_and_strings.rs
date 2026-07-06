/* =============================== Arrays and strings ===============================
Assumptions:
Assume all input string use only the 128 standard ASCII characters.
To have ASCII-only strings in rust the ascii module is used so that the input
has to follow the assumption.
*/

use ascii::{AsciiChar, AsciiStr, AsciiString};

/* Function that returns true if a string has all unique characters and false if not.
If an empty string is given, the function returns true.
Time: O(N), Space: O(1)
*/
pub fn is_unique(string: &AsciiStr) -> bool {
    if string.len() > 128 {
        return false;
    }
    let mut bit_vector = 0u128;
    for &byte in string.as_bytes() {
        let mask = 1u128 << byte;
        if bit_vector & mask != 0 {
            return false;
        }
        bit_vector |= mask;
    }
    true
}

/* Function that returns true if string s1 is permutation of s2 and false if not.
If both strings are empty, the function returns true.
Time: O(N), Space: O(1)
*/
pub fn check_permutation(s1: &AsciiStr, s2: &AsciiStr) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut counts = [0usize; 128];
    for &byte in s1.as_bytes() {
        counts[byte as usize] += 1;
    }
    for &byte in s2.as_bytes() {
        let index = byte as usize;
        if counts[index] == 0 {
            return false;
        }
        counts[index] -= 1;
    }
    true
}

/* Function that, given a list of chars and its length, substitutes in-place
spaces with "%20".
The function assumes that the lenght of the input array of chars is equal to
the final size after the expansion.
Time: O(N), Space: O(1)
*/
pub fn urlify(string: &mut [AsciiChar], true_len: usize) {
    let mut write_id = string.len();
    let ascii_space = AsciiChar::Space;
    let ascii_percent = AsciiChar::Percent;
    let ascii_two = AsciiChar::from_ascii('2').unwrap();
    let ascii_zero = AsciiChar::from_ascii('0').unwrap();
    for read_id in (0..true_len).rev() {
        if string[read_id] == ascii_space {
            string[write_id - 1] = ascii_zero;
            string[write_id - 2] = ascii_two;
            string[write_id - 3] = ascii_percent;
            write_id -= 3;
        } else {
            string[write_id - 1] = string[read_id];
            write_id -= 1;
        }
    }
}

/* Function that given a string return true if it is a permutation of a
palindrome. Palindromes can be not real words with meaning.
The function ignores casing and non-letter characters.
Time: O(N), Space: O(1)
*/
pub fn palindrome_permutation(string: &AsciiStr) -> bool {
    let mut odd_chars = 0u32;
    for &byte in string.as_bytes() {
        if byte.is_ascii_alphabetic() {
            let index = byte.to_ascii_lowercase() - b'a';
            let mask = 1 << index;
            odd_chars ^= mask;
        }
    }
    (odd_chars & odd_chars.wrapping_sub(1)) == 0
}

/* Function that given two strings returns true if one edit or less is required
to get the second string from the first one. The type of possible edits are:
- inserting a character
- removing a character
- replacing a character.
N is length of the shortest string.
Time: O(N), Space: O(1)
*/
pub fn one_away(s1: &AsciiStr, s2: &AsciiStr) -> bool {
    if s1.len().abs_diff(s2.len()) > 1 {
        return false;
    }
    let (shorter, longer) = if s1.len() < s2.len() {
        (s1.as_bytes(), s2.as_bytes())
    } else {
        (s2.as_bytes(), s1.as_bytes())
    };
    let mut idx_s = 0;
    let mut idx_l = 0;
    let mut found_difference = false;
    while idx_s < shorter.len() && idx_l < longer.len() {
        if shorter[idx_s] != longer[idx_l] {
            if found_difference {
                return false;
            }
            found_difference = true;
            if shorter.len() == longer.len() {
                idx_s += 1;
            }
        } else {
            idx_s += 1;
        }
        idx_l += 1;
    }
    true
}

/* Function to compress a string by substituting repeated chars with the count.
Filters out all non-alphabetical characters from the input strings.
If there is no advantage in compression length, return original string.
Time: O(N), Space: O(N)
*/
pub fn string_compression(string: &AsciiStr) -> AsciiString {
    fn num_digits(mut n: usize) -> usize {
        let mut count = 0;
        while n > 0 {
            count += 1;
            n /= 10;
        }
        count.max(1)
    }
    let mut runs: Vec<(AsciiChar, usize)> = Vec::new();
    let mut valid_char_count = 0;
    let filtered_chars = string
        .as_slice()
        .iter()
        .copied()
        .filter(|ch| ch.as_char().is_ascii_alphabetic());
    for ch in filtered_chars {
        valid_char_count += 1;
        match runs.last_mut() {
            Some((last_ch, count)) if *last_ch == ch => *count += 1,
            _ => runs.push((ch, 1)),
        }
    }
    if runs.is_empty() {
        return AsciiString::new();
    }
    let compressed_len: usize = runs.iter().map(|&(_, count)| 1 + num_digits(count)).sum();
    if compressed_len >= valid_char_count {
        let mut original = AsciiString::with_capacity(valid_char_count);
        for (ch, count) in runs {
            for _ in 0..count {
                original.push(ch);
            }
        }
        return original;
    }
    let mut compressed = AsciiString::with_capacity(compressed_len);
    for (ch, count) in runs {
        compressed.push(ch);
        compressed.push_str(&AsciiString::from_ascii(count.to_string()).unwrap());
    }
    compressed
}

/* Function to rotate by 90 degrees a NxN matrix of integer values.
The rotation is done in-place to save memory.
Time: O(), Space: O()
*/

/* Function that given a MxN matrix sets rows and columns to 0 where a 0
element is present.
Time: O(), Space: O()
*/

/* Function to check if a string s1 is a rotation of another.
Time: O(), Space: O()
*/

// =============================== Test suite ===============================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        let test_cases = &[
            ("", true),
            ("hello", false),
            ("world", true),
            ("s4fad", true),
            ("hb 627jh=j ()", false),
            ("aA", true),
        ];
        for &(input, expected) in test_cases {
            let ascii_input = AsciiStr::from_ascii(input).unwrap();
            assert_eq!(
                is_unique(ascii_input),
                expected,
                "Test failed for input string: '{}'",
                input
            );
        }
    }

    #[test]
    fn test_check_permutation() {
        let test_cases = &[
            ("", "", true),
            ("abc", "bca", true),
            ("abc", "abcd", false),
            ("abc", "abd", false),
            ("aabb", "bbaa", true),
            ("hello", "olleh", true),
            ("hello", "ollhe!", false),
            ("aA", "Aa", true),
        ];

        for &(s1, s2, expected) in test_cases {
            let a1 = AsciiStr::from_ascii(s1).unwrap();
            let a2 = AsciiStr::from_ascii(s2).unwrap();
            assert_eq!(check_permutation(a1, a2), expected);
        }
    }

    #[test]
    fn test_urlify() {
        let run_test = |input: &str, true_len: usize, buffer_size: usize, expected: &str| {
            let mut ascii_buffer = vec![AsciiChar::Space; buffer_size];
            let input_ascii = AsciiStr::from_ascii(input).unwrap();
            for (i, &char) in input_ascii.as_slice().iter().enumerate() {
                ascii_buffer[i] = char;
            }
            urlify(&mut ascii_buffer, true_len);
            let expected_ascii = AsciiStr::from_ascii(expected).unwrap().as_slice();
            assert_eq!(
                ascii_buffer.as_slice(),
                expected_ascii,
                "Failed for input: '{}'",
                input
            );
        };
        run_test("Mr John Smith", 13, 17, "Mr%20John%20Smith");
        run_test("NoSpacesHere", 12, 12, "NoSpacesHere");
        run_test("   ", 3, 9, "%20%20%20");
        run_test("", 0, 0, "");
        run_test("a  b", 4, 8, "a%20%20b");
        run_test(" a ", 3, 7, "%20a%20");
        run_test("~ !", 3, 5, "~%20!");
    }

    #[test]
    fn test_palindrom_permutation() {
        let test_cases = &[
            ("Tact Coa", true),
            ("hello", false),
            ("", true),
            (" !?., ", true),
            ("A man, a plan, a canal: Panama", true),
            ("aaaa", true),
            ("aaaaa", true),
            ("Tact Coaa", false),
        ];

        for &(input, expected) in test_cases {
            let ascii_input = AsciiStr::from_ascii(input).unwrap();
            assert_eq!(palindrome_permutation(ascii_input), expected);
        }
    }

    #[test]
    fn test_one_away() {
        let test_cases = &[
            ("pale", "pale", true),
            ("pale", "bale", true),
            ("pale", "ple", true),
            ("pales", "pale", true),
            ("pale", "pales", true),
            ("pale", "bake", false),
            ("pale", "p", false),
            ("", "a", true),
            ("a", "", true),
            ("", "", true),
            ("", "ab", false),
        ];

        for &(s1, s2, expected) in test_cases {
            let a1 = AsciiStr::from_ascii(s1).unwrap();
            let a2 = AsciiStr::from_ascii(s2).unwrap();
            assert_eq!(one_away(a1, a2), expected);
        }
    }

    #[test]
    fn test_string_compression() {
        let test_cases = &[
            ("aabcccccaaa", "a2b1c5a3"),
            ("aabbbbccc", "a2b4c3"),
            ("abc", "abc"),
            ("aabb", "aabb"),
            ("aA", "aA"),
            ("", ""),
            ("a", "a"),
            ("a!b?c", "abc"),
            ("a b b c c c", "abbccc"),
            ("a b b b c c c", "a1b3c3"),
            ("12345", ""),
        ];

        for &(input, expected) in test_cases {
            let ascii_input = AsciiStr::from_ascii(input).unwrap();
            let expected_ascii = AsciiString::from_ascii(expected).unwrap();

            assert_eq!(
                string_compression(ascii_input),
                expected_ascii,
                "Test failed for input string: '{}'",
                input
            );
        }
    }
}
