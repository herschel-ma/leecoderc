use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        // 使用栈来处理嵌套的括号，每个元素是一个哈希表，用于存储当前层级的原子计数
        let mut stack = vec![HashMap::new()];
        let mut i = 0;
        // 将输入字符串转换为字符数组，方便随机访问
        let chars: Vec<char> = formula.chars().collect();
        while i < chars.len() {
            match chars[i] {
                '(' => {
                    // 遇到左括号，在栈中添加一个新的哈希表
                    stack.push(HashMap::new());
                    i += 1;
                }
                ')' => {
                    // 遇到右括号，弹出栈顶的哈希表
                    let top = stack.pop().unwrap();
                    i += 1;
                    let count = Self::parse_number(&chars, &mut i);
                    // 获取新的栈顶
                    let last = stack.last_mut().unwrap();
                    // 将栈顶的哈希表中的原子计数乘以解析的数字，然后合并到新的栈顶
                    for (atom, freq) in top {
                        *last.entry(atom).or_insert(0) += freq * count;
                    }
                }
                'A'..='Z' => {
                    // 遇到大写字母，解析原子名称
                    let atom = Self::parse_atom(&chars, &mut i);
                    // 解析原子名称后可能限随的数字
                    let count = Self::parse_number(&chars, &mut i);
                    // 更新栈顶的原子计数
                    *stack.last_mut().unwrap().entry(atom).or_insert(0) += count;
                }
                _ => i += 1, // 忽略其他的字符
            }
        }
        // 将最终的原子计数结果转换为向量，并按原子名称排序
        let mut atoms: Vec<_> = stack[0].clone().into_iter().collect();
        atoms.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        // 格式化输出结果
        atoms
            .into_iter()
            .map(|(atom, count)| {
                if count == 1 {
                    atom
                } else {
                    format!("{}{}", atom, count)
                }
            })
            .collect()
    }

    // 解析原子名称
    pub(crate) fn parse_atom(chars: &[char], i: &mut usize) -> String {
        let mut atom = String::new();
        atom.push(chars[*i]);
        *i += 1;
        // 继续解析小写字母
        while *i < chars.len() && chars[*i].is_ascii_lowercase() {
            atom.push(chars[*i]);
            *i += 1;
        }
        atom
    }

    pub(crate) fn parse_number(chars: &[char], i: &mut usize) -> i32 {
        let mut number = 0;
        while *i < chars.len() && chars[*i].is_ascii_digit() {
            number = number * 10 + (chars[*i] as i32 - '0' as i32);
            *i += 1;
        }
        // 如果没有解析到数字，返回1（因为默认数量是1）
        if number == 0 {
            1
        } else {
            number
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let formula = "H2O".to_string();
        let output = "H2O".to_string();
        assert_eq!(Solution::count_of_atoms(formula), output)
    }

    #[test]
    fn test_case_2() {
        let formula = "Mg(OH)2".to_string();
        let output = "H2MgO2".to_string();
        assert_eq!(Solution::count_of_atoms(formula), output)
    }

    #[test]
    fn test_case_3() {
        let formula = "K4(ON(SO3)2)2".to_string();
        let output = "K4N2O14S4".to_string();
        assert_eq!(Solution::count_of_atoms(formula), output)
    }
}
