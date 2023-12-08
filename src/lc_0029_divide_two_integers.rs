pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == divisor {
        return 1;
    }
    let is_negative = dividend.is_negative() ^ divisor.is_negative();
    let (mut a, b) = (dividend.unsigned_abs(), divisor.unsigned_abs());
    let mut ans = 0;

    while a >= b {
        let mut q = 0;
        while a > (b << (q + 1)) {
            q += 1;
        }
        ans += 1 << q;
        a -= b << q;
    }

    if ans == 1 << 31 && !is_negative {
        return i32::MAX;
    }

    match is_negative {
        true => -ans,
        false => ans,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let dividend = 10;
        let divisor = 3;
        let res = 3;
        assert_eq!(divide(dividend, divisor), res)
    }

    #[test]
    fn test_case_2() {
        let dividend = 7;
        let divisor = -3;
        let res = -2;
        assert_eq!(divide(dividend, divisor), res)
    }
}
