use std::cmp::Ordering;

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn combi(ans: &mut Vec<Vec<i32>>, buffer: &mut Vec<i32>, candidates: &[i32], remain: i32) {
        match remain.cmp(&0) {
            Ordering::Equal => {
                ans.push(buffer.to_vec());
            }
            Ordering::Greater => {
                for i in 0..candidates.len() {
                    if i > 0 && candidates[i] == candidates[i - 1] {
                        continue;
                    }
                    buffer.push(candidates[i]);
                    combi(ans, buffer, &candidates[i + 1..], remain - candidates[i]);
                    buffer.pop();
                }
            }
            Ordering::Less => {}
        }
    }

    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut buffer: Vec<i32> = Vec::new();
    let mut candidates = candidates.clone();
    candidates.sort();
    combi(&mut ans, &mut buffer, &candidates, target);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let output = Vec::from([
            Vec::from([1, 1, 6]),
            Vec::from([1, 2, 5]),
            Vec::from([1, 7]),
            Vec::from([2, 6]),
        ]);
        assert_eq!(combination_sum2(candidates, target), output);
    }
    #[test]
    fn test_case_2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let output = Vec::from([Vec::from([1, 2, 2]), Vec::from([5])]);
        assert_eq!(combination_sum2(candidates, target), output);
    }
}
