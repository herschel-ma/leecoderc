pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
    // create graph represents prerequisites
    let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
    for rel in relations {
        if let [prev, next] = rel.as_slice() {
            graph[(prev - 1) as usize].push(next - 1);
        }
    }
    // create a memoization table(an array)
    // to sotre the minimum time needed to complate each course.
    let mut memo = vec![-1; n as usize];
    let mut over_all_time = 0;
    for course in 0..n {
        over_all_time =
            over_all_time.max(calculate_complete_course(course, &mut memo, &graph, &time))
    }
    over_all_time
}
// define a recusive function to calculate the minimum time
// to complete a course
fn calculate_complete_course(
    course: i32,
    memo: &mut Vec<i32>,
    graph: &Vec<Vec<i32>>,
    time: &Vec<i32>,
) -> i32 {
    if memo[course as usize] != -1 {
        return memo[course as usize];
    }

    if graph[course as usize].is_empty() {
        memo[course as usize] = time[course as usize];
        return memo[course as usize];
    }

    let mut max_prerequisite_time = 0;
    for prereq in &graph[course as usize] {
        max_prerequisite_time =
            max_prerequisite_time.max(calculate_complete_course(*prereq, memo, graph, time));
    }
    memo[course as usize] = max_prerequisite_time + time[course as usize];
    memo[course as usize]
}

#[cfg(test)]
mod tests {
    use crate::minimum_time;

    #[test]
    fn case() {
        let n = 3;
        let relations = vec![vec![1, 3], vec![2, 3]];
        let time = vec![3, 2, 5];
        let out_put = 8;
        assert_eq!(minimum_time(n, relations, time), out_put);
    }

    #[test]
    fn case_2() {
        let n = 5;
        let relations = vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]];
        let time = vec![1, 2, 3, 4, 5];
        let out_put = 12;
        assert_eq!(minimum_time(n, relations, time), out_put);
    }
}
