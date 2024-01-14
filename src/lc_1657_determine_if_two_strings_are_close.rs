use crate::Solution;

impl Solution {
    /// Two strings considered `close` if you can attain one from
    /// the other using the following operations:
    ///
    /// - Operation 1: Swap any two existing characters
    ///     - For example, `abcde -> aecdb`
    /// - Operation 2: Transform every occurrence of one existing
    /// character into another existing character, and do the same
    /// with the other character.
    ///     - For example, `aacabb -> bbcbaa`
    /// (all a's turn into b's, and all b's turn into a's)
    ///
    /// You can use the operations on either string as many times as
    /// necessary.
    /// Given two strings, word1 and word2, return true if word1 and
    /// word2 are close, nd false otherwise.
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let (mut c1, mut c2) = ([0; 26], [0; 26]);
        word1.chars().zip(word2.chars()).for_each(|(x, y)| {
            let (i1, i2) = ((x as u8 - b'a') as usize, (y as u8 - b'a') as usize);
            c1[i1] += 1;
            c2[i2] += 1;
        });
        // c1: [2,3,1,0...]
        // c2: [1,2,3,0...]
        for i in 0..26 {
            if (c1[i] != 0 && c2[i] == 0) || (c2[i] != 0 && c1[i] == 0) {
                return false;
            }
        }

        c1.sort();
        c2.sort();
        c1 == c2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_close_strings_1() {
        let word1 = "abc".to_string();
        let word2 = "bca".to_string();
        assert!(Solution::close_strings(word1, word2));
    }

    #[test]
    fn test_close_strings_2() {
        let word1 = "a".to_string();
        let word2 = "aa".to_string();
        assert!(!Solution::close_strings(word1, word2));
    }

    #[test]
    fn test_close_strings_3() {
        let word1 = "cabbba".to_string();
        let word2 = "abbccc".to_string();
        assert!(Solution::close_strings(word1, word2));
    }

    #[test]
    fn should_be_close() {
        let word1 = "cabbba".to_string();
        let word2 = "aabbss".to_string();
        assert!(!Solution::close_strings(word1, word2));
    }

    #[test]
    fn wrong_ans() {
        let word1 = "uau".to_string();
        let word2 = "ssx".to_string();
        assert!(!Solution::close_strings(word1, word2));
    }
}

