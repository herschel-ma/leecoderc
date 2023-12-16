pub fn insert_1(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut to_insert = new_interval;
    let mut pushed = false;

    for entry in intervals {
        if entry.is_empty() {
            continue;
        }
        if entry[0] > to_insert[1] {
            if !pushed {
                result.push(to_insert.clone());
                pushed = true;
            }
            result.push(entry);
        } else if entry[1] < to_insert[0] {
            result.push(entry);
        } else {
            to_insert[0] = to_insert[0].min(entry[0]);
            to_insert[1] = to_insert[1].max(entry[1]);
        }
    }
    if !pushed {
        result.push(to_insert);
    }
    result
}

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut new_interval = new_interval;
    for interval in intervals {
        // 如果新区间完全在当前区间之前，将新区间加入到 result，并更新新区间为当前区间
        if interval[0] > new_interval[1] {
            result.push(new_interval);
            new_interval = interval;
        // 如果当前区间在新区间之前，将当前区间加入 result
        } else if interval[1] < new_interval[0] {
            result.push(interval)
        // 如果当前区间和新区间有重叠
        } else {
            new_interval[0] = interval[0].min(new_interval[0]);
            new_interval[1] = interval[1].max(new_interval[1]);
        }
    }

    result.push(new_interval);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        )
    }

    #[test]
    fn ex2() {
        assert_eq!(
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        )
    }

    #[test]
    fn ex3() {
        assert_eq!(insert(vec![], vec![5, 7]), vec![vec![5, 7]])
    }
}
