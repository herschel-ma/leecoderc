use std::iter::once;

pub fn count_and_say_other(n: i32) -> String {
    let mut curr: Vec<u8> = vec![1];
    for _ in 1..n {
        let mut next = vec![];
        let mut slow = 0;
        for fast in 0..=curr.len() {
            if fast == curr.len() || curr[slow] != curr[fast] {
                next.extend(once((fast - slow) as u8).chain(once(curr[slow])));
                slow = fast;
            }
        }
        curr = next;
    }
    curr.into_iter()
        .map(|digit| (digit + b'0') as char)
        .collect()
}

pub fn count_and_say(n: i32) -> String {
    (1..n)
        .fold(vec![1], |curr, _| {
            let mut next = vec![];
            let mut slow = 0;
            for fast in 0..=curr.len() {
                if fast == curr.len() || curr[slow] != curr[fast] {
                    next.extend(once((fast - slow) as u8).chain(once(curr[slow])));
                    slow = fast;
                }
            }
            next
        })
        .into_iter()
        .map(|digit| (digit + b'0') as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 1;
        let output = "1".to_string();
        assert_eq!(count_and_say(n), output)
    }

    #[test]
    fn test_case_2() {
        let n = 4;
        let output = "1211".to_string();
        assert_eq!(count_and_say(n), output)
    }
}
