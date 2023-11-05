pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    if k == 1 {
        return arr[0].max(arr[1]);
    }
    if k as usize >= arr.len() {
        return arr.into_iter().max().unwrap();
    }
    let mut consecutive_win = 0;
    let mut winner = arr[0];
    for &ele in &arr[1..] {
        if ele > winner {
            winner = ele;
            consecutive_win = 1;
        } else {
            consecutive_win += 1;
        }
        if consecutive_win == k {
            return winner;
        }
    }
    winner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let arr = vec![2, 1, 3, 5, 4, 6, 7];
        let k = 2;
        assert_eq!(get_winner(arr, k), 5);
    }

    #[test]
    fn test_case2() {
        let arr = vec![3, 2, 1];
        let k = 10;

        assert_eq!(get_winner(arr, k), 3)
    }
}
