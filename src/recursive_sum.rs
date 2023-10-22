pub fn sum(array: &mut Vec<i32>) -> i32 {
    if array.len() == 0 {
        return 0;
    }
    return array.pop().unwrap() + sum(array);
}

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn case() {
        let mut array = vec![2, 4, 6];
        assert_eq!(sum(&mut array), 12);
    }
}