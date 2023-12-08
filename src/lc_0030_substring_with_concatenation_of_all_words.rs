use std::collections::HashMap;

pub fn remove_map(map: &mut HashMap<String, i32>, s: String) {
    if let Some(counter) = map.get_mut(&s) {
        *counter -= 1;
        if *counter == 0 {
            map.remove(&s);
        }
    } else {
        map.insert(s, -1);
    }
}

pub fn insert_map(map: &mut HashMap<String, i32>, s: String) {
    if let Some(counter) = map.get_mut(&s) {
        *counter += 1;
        if *counter == 0 {
            map.remove(&s);
        }
    } else {
        map.insert(s, 1);
    }
}

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let n = words.len();
    let m = words[0].len();
    let len = s.len();
    let mut res = vec![];
    for i in 0..m {
        let mut map = HashMap::new();
        for word in words.clone() {
            remove_map(&mut map, word);
        }
        let mut start = i;
        loop {
            if start + n * m > len {
                break;
            }
            if start == i {
                for index in 0..n {
                    insert_map(
                        &mut map,
                        String::from(&s[start + index * m..start + (index + 1) * m]),
                    )
                }
            } else {
                remove_map(&mut map, String::from(&s[start - m..start]));
                insert_map(
                    &mut map,
                    String::from(&s[start + (n - 1) * m..start + n * m]),
                );
            }
            if map.is_empty() {
                res.push(start as i32);
            }

            start += m;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = String::from("barfoothefoobarman");
        let words = vec![String::from("foo"), String::from("bar")];
        let res = vec![0, 9];
        assert_eq!(find_substring(s, words), res);
    }

    #[test]
    fn test_case_2() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![
            String::from("word"),
            String::from("good"),
            String::from("best"),
            String::from("word"),
        ];
        let res = vec![];
        assert_eq!(find_substring(s, words), res)
    }
}
