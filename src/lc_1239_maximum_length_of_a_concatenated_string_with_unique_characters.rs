use crate::Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let sets = arr
            .into_iter()
            .filter_map(|s| {
                let mut bitset = 0;
                for bm in s.bytes().map(|b| 1 << (b - b'a')) {
                    if bitset & bm != 0 {
                        return None;
                    } else {
                        bitset |= bm;
                    }
                }
                Some(bitset)
            })
            .collect::<Vec<_>>();

        let mut stack = vec![(0_i32, 0)];
        let mut rez = 0;

        while let Some((bitset, i)) = stack.pop() {
            if i == sets.len() {
                rez = rez.max(bitset.count_ones());
            } else {
                if bitset & sets[i] == 0 {
                    stack.push((bitset | sets[i], i + 1));
                }
                stack.push((bitset, i + 1));
            }
        }
        rez as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let arr = vec![String::from("un"), String::from("iq"), String::from("ue")];
        assert_eq!(Solution::max_length(arr), 4);
    }

    #[test]
    fn test_case_2() {
        let arr = vec![
            String::from("cha"),
            String::from("r"),
            String::from("act"),
            String::from("ers"),
        ];
        assert_eq!(Solution::max_length(arr), 6);
    }

    #[test]
    fn test_case_3() {
        let arr = vec![String::from("abcdefghijklmnopqrstuvwxyz")];
        assert_eq!(Solution::max_length(arr), 26);
    }
}
