use crate::Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        s.chars()
            .fold(vec![], |mut stk, c| match c {
                ')' => {
                    let mut cur = vec![];
                    while let Some(x) = stk.pop() {
                        if x == '(' {
                            break;
                        }
                        cur.push(x);
                    }

                    for x in cur {
                        stk.push(x);
                    }
                    stk
                }
                _ => {
                    stk.push(c);
                    stk
                }
            })
            .into_iter()
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("(abcd)");
        let output = String::from("dcba");
        assert_eq!(Solution::reverse_parentheses(s), output);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("(u(love)i)");
        let output = String::from("iloveu");
        assert_eq!(Solution::reverse_parentheses(s), output);
    }

    #[test]
    fn test_case_3() {
        let s = String::from("(ed(et(oc))el)");
        let output = String::from("leetcode");
        assert_eq!(Solution::reverse_parentheses(s), output);
    }
}
