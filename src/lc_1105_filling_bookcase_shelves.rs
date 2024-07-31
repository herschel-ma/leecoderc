use crate::Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![i32::MAX; n];
        for i in 0..n {
            let mut w = 0;
            let mut h = 0;
            for j in (0..=i).rev() {
                w += books[j][0];
                if w > shelf_width {
                    break;
                };
                h = h.max(books[j][1]);
                dp[i] = dp[i].min(h + if j > 0 { dp[j - 1] } else { 0 })
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let books = vec![
            vec![1, 1],
            vec![2, 3],
            vec![2, 3],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 2],
        ];
        let shelf_width = 4;
        assert_eq!(Solution::min_height_shelves(books, shelf_width), 6);
    }

    #[test]
    fn test_case_2() {
        let books = vec![vec![1, 3], vec![2, 4], vec![3, 2]];
        let shelf_width = 6;
        assert_eq!(Solution::min_height_shelves(books, shelf_width), 4);
    }
}
