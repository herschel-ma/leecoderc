use crate::Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        fn helper(num: usize) -> String {
            const TENS: [&str; 10] = [
                "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty",
                "Ninety",
            ];
            const NUMBERS: [&str; 20] = [
                "",
                "One",
                "Two",
                "Three",
                "Four",
                "Five",
                "Six",
                "Seven",
                "Eight",
                "Nine",
                "Ten",
                "Eleven",
                "Twelve",
                "Thirteen",
                "Fourteen",
                "Fifteen",
                "Sixteen",
                "Seventeen",
                "Eighteen",
                "Nineteen",
            ];
            let mut s = String::new();
            match num {
                n if n < 20 => s.push_str(NUMBERS[n]),
                n if n < 100 => {
                    s.push_str(TENS[n / 10]);
                    s.push(' ');
                    s.push_str(NUMBERS[n % 10]);
                }
                n if n < 1000 => {
                    s.push_str(NUMBERS[n / 100]);
                    s.push_str(" Hundred ");
                    s.push_str(&helper(n % 100));
                }
                n if n < 1000000 => {
                    s.push_str(&helper(n / 1000));
                    s.push_str(" Thousand ");
                    s.push_str(&helper(n % 1000));
                }
                n if n < 1000000000 => {
                    s.push_str(&helper(n / 1000000));
                    s.push_str(" Million ");
                    s.push_str(&helper(n % 1000000));
                }
                _ => {
                    s.push_str(&helper(num / 1000000000));
                    s.push_str(" Billion ");
                    s.push_str(&helper(num % 1000000000));
                }
            }
            s.trim().to_string()
        }
        helper(num as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let num = 123;
        assert_eq!(
            "One Hundred Twenty Three".to_string(),
            Solution::number_to_words(num)
        );
    }

    #[test]
    fn test_case_2() {
        let num = 12345;
        assert_eq!(
            "Twelve Thousand Three Hundred Forty Five".to_string(),
            Solution::number_to_words(num)
        );
    }

    #[test]
    fn test_case_3() {
        let num = 1234567;
        assert_eq!(
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string(),
            Solution::number_to_words(num)
        );
    }
}
