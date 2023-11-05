pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
    let left = left.iter().max().map_or(0, |e| *e);
    let right = right.iter().min().map_or(n, |e| *e);
    left.max(n - right)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case1() {
        let n = 4;
        let left = vec![4, 3];
        let right = vec![0, 1];
        assert_eq!(get_last_moment(n, left, right), 4);
    }

    #[test]
    fn test_case2() {
        let n = 7;
        let left = vec![];
        let right = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(get_last_moment(n, left, right), 7);
    }

    #[test]
    fn test_case3() {
        let n = 7;
        let left = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let right = vec![];
        assert_eq!(get_last_moment(n, left, right), 7)
    }
}
