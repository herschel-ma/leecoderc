use std::cmp::Ordering;

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    let mut t: Vec<i32>;

    fn dfs(candidates: &[i32], ans: &mut Vec<Vec<i32>>, t: &mut Vec<i32>, target: i32) {
        t.push(candidates[0]);
        match t.iter().sum::<i32>().cmp(&target) {
            Ordering::Equal => ans.push(t.clone()),
            Ordering::Less => {
                for i in 0..candidates.len() {
                    dfs(&candidates[i..], ans, t, target)
                }
            }
            Ordering::Greater => {}
        }
        t.pop();
    }

    for i in 0..candidates.len() {
        t = vec![];
        dfs(&candidates[i..], &mut ans, &mut t, target);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let output = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(combination_sum(candidates, target), output);
    }

    #[test]
    fn test_case_2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let output = Vec::from([
            Vec::from([2, 2, 2, 2]),
            Vec::from([2, 3, 3]),
            Vec::from([3, 5]),
        ]);
        assert_eq!(combination_sum(candidates, target), output);
    }

    #[test]
    fn test_case_3() {
        let candidates = vec![2];
        let target = 1;
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(combination_sum(candidates, target), output);
    }
}
