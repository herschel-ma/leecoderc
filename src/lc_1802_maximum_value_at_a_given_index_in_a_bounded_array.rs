//! You are given three positive integers: n, index and max_sum.
//! You want to construct an array nums(0-indexed) that statisfies
//! the following conditions:
//! * `nums.length == n`
//! * `nums[i]` is a positive integer where `0 <= i < n`.
//! * `abs(nums[i] - nums[i + 1]) <= 1` where `0 <= i < n -1`.
//! * The sum of all the elements of `nums` does not exceed `max_sum`.
//! * `nums[index]` is **maximized**.

/// max_vlue return a maximum i32
/// [题解](https://leetcode.cn/problems/maximum-value-at-a-given-index-in-a-bounded-array/solutions/2045235/er-fen-cha-zhao-tan-xin-by-lxk1203-a883/)
///
pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    let mut left = 1; // 二分查找左边界， nums[index] 可取最小值1
    let mut right = max_sum; // 二分查找右边界，nums[index] 可取最大值max_sum
    let mut ans = n;
    while left <= right {
        let mid = left + (right - left) / 2;
        // 左区间
        // [0..index]
        //  index-(m-1)个1,
        // [1, 1, ..      ,1,             2,..,m-1,m]
        // [0, 1, ..      ,i-(m-1), i-(m-2),..,i-1,i]
        let left_sum: i32 = if index >= mid - 1 {
            mid * (mid - 1) / 2 + index - (mid - 1)
        } else {
            //  index个数
            // [m-i, m-i+1,..., m-1, m]
            // [0  ,     1,..., i-1, i]
            (mid - index + mid - 1) * index / 2
        };

        // 右区间
        // [index+1..n)
        //  m - 1 个数................., n-1-i-(m-1)个1           // [m, m-1, ...,2      , 1     , ..., 1,   1]
        // [i, i+1, ...,i+(m-2),i+(m-1),...,n-2, n-1]
        let right_sum: i32 = if n - 1 - index >= mid - 1 {
            mid * (mid - 1) / 2 + (n - 1 - index) - (mid - 1)
        } else {
            //     n-1-index个数..
            // [m, m-1, ..., m-(n-1-i+1), m-(n-1-i)]
            // [i, i+1, ...n-2,n-1]
            (mid - 1 + mid - (n - 1 - index)) * (n - 1 - index) / 2
        };

        if (mid + left_sum + right_sum) > max_sum {
            right = mid - 1;
        } else {
            ans = mid;
            left = mid + 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let n = 4;
        let index = 2;
        let max_sum = 6;
        assert_eq!(max_value(n, index, max_sum), 2)
    }

    #[test]
    fn test_case2() {
        let n = 6;
        let index = 1;
        let max_sum = 10;
        assert_eq!(max_value(n, index, max_sum), 3)
    }
}
