use crate::Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut fives, mut tens) = (0, 0);

        for bill in bills {
            match bill {
                5 => fives += 1,
                10 => {
                    if fives == 0 {
                        return false;
                    }
                    fives -= 1;
                    tens += 1;
                }
                20 => {
                    if tens > 0 && fives > 0 {
                        tens -= 1;
                        fives -= 1;
                    } else if fives >= 3 {
                        fives -= 3;
                    } else {
                        return false;
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let bills = vec![5, 5, 5, 10, 20];
        assert!(Solution::lemonade_change(bills));
    }

    #[test]
    fn test_case_2() {
        let bills = vec![5, 5, 10, 10, 20];
        assert!(!Solution::lemonade_change(bills));
    }
}
