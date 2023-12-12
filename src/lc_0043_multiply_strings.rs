pub fn multiply_o(num1: String, num2: String) -> String {
    let sz1 = num1.len();
    let sz2 = num2.len();
    let mut ret: Vec<u32> = vec![0; sz1 + sz2];

    let num1: Vec<u32> = num1
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let num2: Vec<u32> = num2
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for i in 0..sz1 {
        for j in 0..sz2 {
            let mul = num1[i] * num2[j];
            let sum = ret[i + j] + mul;
            ret[i + j] = sum % 10;
            ret[i + j + 1] += sum / 10;
        }
    }

    while ret.len() > 1 && ret.last() == Some(&0) {
        ret.pop();
    }

    ret.into_iter()
        .rev()
        .map(|d| std::char::from_digit(d, 10).unwrap())
        .collect()
}
pub fn multiply(num1: String, num2: String) -> String {
    let mut ans = vec![0; num1.len() + num2.len()];
    for (i, ch1) in num1.chars().rev().peekable().enumerate() {
        for (j, ch2) in num2.chars().rev().peekable().enumerate() {
            let a = ch1.to_digit(10).unwrap();
            let b = ch2.to_digit(10).unwrap();

            let low = (a * b + ans[i + j]) % 10;
            let high = (a * b + ans[i + j]) / 10;
            ans[i + j] = low;
            ans[i + j + 1] += high;
        }
    }
    while ans.len() > 1 && ans.last() == Some(&0) {
        ans.pop();
    }
    ans.into_iter()
        .rev()
        .map(|d| std::char::from_digit(d, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let num1 = String::from("2");
        let num2 = String::from("3");
        let output = String::from("6");
        assert_eq!(multiply(num1, num2), output);
    }

    #[test]
    fn test_case_2() {
        let num1 = String::from("123");
        let num2 = String::from("456");
        let output = String::from("56088");
        assert_eq!(multiply(num1, num2), output);
    }
}
