use crate::Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        mut healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        // 创建机器人索引数组并按位置排序
        let mut robots = (0..n).collect::<Vec<usize>>();
        robots.sort_by(|a, b| positions[*a].cmp(&positions[*b]));

        // 用于存储存活的机器人
        let mut stack: Vec<usize> = Vec::new();

        // 遍历排序后的机器人
        for &i in &robots {
            if directions.as_bytes()[i] == b'R' {
                // 如果机器人向右移动，将其索引推入栈中
                stack.push(i);
            } else {
                // 如果机器人向左移动，与栈中的元素碰撞
                while !stack.is_empty() && healths[i] > 0 {
                    let j = *stack.last().unwrap();
                    match healths[i].cmp(&healths[j]) {
                        // 如果向左移动的健康值更小
                        std::cmp::Ordering::Less => {
                            healths[j] -= 1;
                            healths[i] = 0;
                            break;
                        }
                        // 如果健康值相等，两个机器人都被摧毁
                        std::cmp::Ordering::Equal => {
                            healths[i] = 0;
                            healths[j] = 0;
                            stack.pop();
                            break;
                        }
                        std::cmp::Ordering::Greater => {
                            healths[j] = 0;
                            healths[i] -= 1;
                            stack.pop();
                        }
                    }
                }
            }
        }
        // 计算存活机器人的最终健康值
        healths.into_iter().filter(|&h| h > 0).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let positions = vec![5, 4, 3, 2, 1];
        let healths = vec![2, 17, 9, 15, 10];
        let directions = String::from("RRRRR");
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            vec![2, 17, 9, 15, 10]
        );
    }

    #[test]
    fn test_case_2() {
        let positions = vec![3, 5, 2, 6];
        let healths = vec![10, 10, 15, 12];
        let directions = String::from("RLRL");
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            vec![14]
        );
    }

    #[test]
    fn test_case_3() {
        let positions = vec![1, 2, 5, 6];
        let healths = vec![10, 10, 11, 11];
        let directions = String::from("RLRL");
        let output: Vec<i32> = vec![];
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            output
        );
    }

    #[test]
    fn test_case_4() {
        let positions = vec![3, 40];
        let healths = vec![49, 11];
        let directions = String::from("LL");
        assert_eq!(
            Solution::survived_robots_healths(positions, healths, directions),
            vec![49, 11]
        );
    }
}
