use crate::Solution;
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let m = meetings.len();
        // sort by meeting time
        meetings.sort_unstable_by_key(|x| x[2]);
        // if perple know secret
        let mut secret = vec![false; n as usize];
        secret[0] = true;
        secret[first_person as usize] = true;

        let mut i = 0;
        while i < m {
            let mut j = i;
            while j + 1 < m && meetings[i][2] == meetings[j + 1][2] {
                j += 1;
            }

            let mut vertices: HashSet<i32> = HashSet::new();
            let mut edges: HashMap<i32, Vec<i32>> = HashMap::new();

            for same_time_meeting in meetings.iter().take(j + 1).skip(i) {
                let x = same_time_meeting[0];
                let y = same_time_meeting[1];

                vertices.insert(x);
                vertices.insert(y);
                edges.entry(x).or_default().push(y);
                edges.entry(y).or_default().push(x);
            }

            let mut q = VecDeque::new();
            vertices.iter().for_each(|&x| {
                if secret[x as usize] {
                    q.push_back(x);
                }
            });

            while !q.is_empty() {
                let u = q.pop_front().unwrap();
                for &v in &edges[&u] {
                    if !secret[v as usize] {
                        secret[v as usize] = true;
                        q.push_back(v);
                    }
                }
            }

            i = j + 1
        }

        secret
            .into_iter()
            .enumerate()
            .filter(|x| x.1)
            .map(|x| x.0 as i32)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 6;
        let meetings = vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]];
        let first_person = 1;
        assert_eq!(
            Solution::find_all_people(n, meetings, first_person),
            vec![0, 1, 2, 3, 5]
        );
    }

    #[test]
    fn test_case_2() {
        let n = 4;
        let meetings = vec![vec![1, 2, 2], vec![3, 1, 3], vec![0, 3, 3]];
        let first_person = 3;
        assert_eq!(
            Solution::find_all_people(n, meetings, first_person),
            vec![0, 1, 3]
        );
    }

    #[test]
    fn test_case_3() {
        let n = 5;
        let meetings = vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]];
        let first_person = 1;
        assert_eq!(
            Solution::find_all_people(n, meetings, first_person),
            vec![0, 1, 2, 3, 4]
        );
    }
}
