pub fn my_pow(x: f64, n: i32) -> f64 {
    fn pow(mut x: f64, mut n: i64) -> f64 {
        let mut ans = 1.0;
        while n != 0 {
            if n & 0x1 == 1 {
                // even
                ans *= x;
            }
            x *= x;
            n >>= 1;
        }
        ans
    }

    let n = n as i64;
    if n >= 0 {
        pow(x, n)
    } else {
        1.0_f64 / pow(x, -n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    #[test]
    fn test_case_1() {
        let x = 2.000_00f64;
        let n = 10;
        assert_ulps_eq!(my_pow(x, n), 1_024.000_00, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_case_2() {
        let x = 2.100_00f64;
        let n = 3;
        assert_ulps_eq!(my_pow(x, n), 9.261_00, epsilon = f64::EPSILON);
    }

    #[test]
    fn test_case_3() {
        let x = 2.000_00f64;
        let n = -2;
        assert_ulps_eq!(my_pow(x, n), 0.25, epsilon = f64::EPSILON);
    }
}
