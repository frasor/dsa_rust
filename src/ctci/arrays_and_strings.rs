/* =============================== Arrays and strings ===============================
Assumptions:
Assume all input string use only the 128 standard ASCII characters.
Since Rust strings are 4 bytes and to model ASCII only 1 byte is needed, the
functions take u8 byte strings as input. This allows to save space and enforce the
constraint.
The check for invalid input (e.g. non-ASCII) is left to the user.
*/

/* Function that returns true if a string has all unique characters and false if not.
If an empty string is given, the function returns true.
Time: O(N), Space: O(1)
*/
pub fn is_unique(string: &[u8]) -> bool {
    if string.len() > 128 {
        return false;
    }
    let mut bit_vector = 0u128;
    for &byte in string {
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
pub fn check_permutation(s1: &[u8], s2: &[u8]) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut counts = [0usize; 128];
    for &byte in s1 {
        counts[byte as usize] += 1;
    }
    for &byte in s2 {
        let index = byte as usize;
        if counts[index] == 0 {
            return false;
        }
        counts[index] -= 1;
    }
    true
}

/* Function that, given a list of chars and its lenght, substitutes in-place
spaces with "%20" returning the reference to the previously modified list.
The function assumes that the given list of bytes has enough left spaces to move
characters to the right.
Time: O(N), Space: O(1)
*/
pub fn urlify(string: &mut [u8], true_len: usize) {
    let mut write_id = string.len();

    for read_id in (0..true_len).rev() {
        if string[read_id] == b' ' {
            string[write_id - 1] = b'0';
            string[write_id - 2] = b'2';
            string[write_id - 3] = b'%';
            write_id -= 3;
        } else {
            string[write_id - 1] = string[read_id];
            write_id -= 1;
        }
    }
}

// =============================== Test suite ===============================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        let test_cases: &[(&[u8], bool)] = &[
            (b"", true),
            (b"hello", false),
            (b"world", true),
            (b"s4fad", true),
            (b"hb 627jh=j ()", false),
            (b"aA", true),
        ];

        for (input, expected) in test_cases {
            let input_str = std::str::from_utf8(input).unwrap_or("<invalid utf8>");
            assert_eq!(
                is_unique(input),
                *expected,
                "Test failed for input string: '{}'",
                input_str
            );
        }
    }

    #[test]
    fn test_check_permutation() {
        let test_cases: &[(&[u8], &[u8], bool)] = &[
            (b"", b"", true),
            (b"abc", b"bca", true),
            (b"abc", b"abcd", false),
            (b"abc", b"abd", false),
            (b"aabb", b"bbaa", true),
            (b"hello", b"olleh", true),
            (b"hello", b"ollhe!", false),
            (b"aA", b"Aa", true),
        ];

        for (s1, s2, expected) in test_cases {
            let s1_str = std::str::from_utf8(s1).unwrap_or("<invalid utf8>");
            let s2_str = std::str::from_utf8(s2).unwrap_or("<invalid utf8>");
            assert_eq!(
                check_permutation(s1, s2),
                *expected,
                "Failed permutation check for ('{}', '{}')",
                s1_str,
                s2_str
            );
        }
    }

    #[test]
    fn test_urlify() {
        let run_test = |input: &str, true_len: usize, buffer_size: usize, expected: &str| {
            let mut bytes: Vec<u8> = input.as_bytes().to_vec();
            bytes.resize(buffer_size, b' ');
            urlify(&mut bytes, true_len);
            let result_string = String::from_utf8(bytes).expect("Invalid UTF-8");
            assert_eq!(result_string, expected, "Failed for input: '{}'", input);
        };

        run_test("Mr John Smith", 13, 17, "Mr%20John%20Smith");
        run_test("NoSpacesHere", 12, 12, "NoSpacesHere");
        run_test("   ", 3, 9, "%20%20%20");
        run_test("", 0, 0, "");
        run_test("a  b", 4, 8, "a%20%20b");
        run_test(" a ", 3, 7, "%20a%20");
        run_test("~ !", 3, 5, "~%20!");
    }
}
