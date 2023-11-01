pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(pref.len());
    res.push(pref[0]);

    for i in 1..pref.len() {
        res.push(pref[i - 1] ^ pref[i])
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let pref = vec![5, 2, 0, 3, 1];
        let array = vec![5, 7, 2, 3, 2];
        assert_eq!(find_array(pref), array);
    }

    #[test]
    fn case2() {
        let pref = vec![13];
        let array = vec![13];
        assert_eq!(find_array(pref), array);
    }
}
