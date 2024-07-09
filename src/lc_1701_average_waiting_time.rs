use crate::Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut current_time = 0_f64;
        let mut total_waiting_time = 0_f64;
        for customer in customers.iter() {
            let arrival_time = customer[0] as f64;
            let service_time = customer[1] as f64;

            // 更新当前时间
            current_time = current_time.max(arrival_time);
            // 计算顾客等待时间
            let wait_time = (current_time - arrival_time) + service_time;
            total_waiting_time += wait_time;

            // 更新当前时间为服务结束时间
            current_time += service_time;
        }
        // 计算平均等待时间
        total_waiting_time / customers.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let customes = vec![vec![1, 2], vec![2, 5], vec![4, 3]];
        assert_eq!(Solution::average_waiting_time(customes), 5.);
    }

    #[test]
    fn test_case_2() {
        let customes = vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]];
        assert_eq!(Solution::average_waiting_time(customes), 3.25);
    }
}
