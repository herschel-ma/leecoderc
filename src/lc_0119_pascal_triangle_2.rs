pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut current = vec![0; row_index as usize + 1];
    let mut prev = vec![0; row_index as usize + 1];

    current[0] = 1;
    for row in 1..=row_index as usize {
        std::mem::swap(&mut current, &mut prev);
        current[0] = prev[0]; 
        for i in 1..row as usize + 1 {
            current[i] = prev[i] + prev[i - 1];
        }
    }
    current.to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1])
    }
    
    #[test]
    fn case2() {
        assert_eq!(get_row(0), vec![1])
    }
    
    #[test]
    fn case3() {
        assert_eq!(get_row(1), [1, 1])
    }
}
