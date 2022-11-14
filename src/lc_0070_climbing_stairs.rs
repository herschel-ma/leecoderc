pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 3 {
            return n;
        }
        let mut f1 = 2;
        let mut f2 = 3;
        let mut re = 0;
        for _i in 4..=n {
            re = f1 + f2;
            f1 = f2;
            f2 = re;
        }
        return re;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::climb_stairs(3), 3)
    }
}
