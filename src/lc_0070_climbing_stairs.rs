pub fn climb_stairs(n: i32) -> i32 {
    if n <= 3 {
        return n;
    }
    let mut f1 = 2;
    let mut f2 = 3;
    let mut re = 0;
    // need return how many **ways** to reach top with n steps
    // so f(n) = f(n-2) + f(n-1)
    // because of you can forword 1 or 2 steps.
    for _i in 4..=n {
        re = f1 + f2;
        f1 = f2;
        f2 = re;
    }
    re
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn ex2() {
        assert_eq!(climb_stairs(3), 3)
    }
}
