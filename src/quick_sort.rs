pub fn quick_sort(mut array: Vec<i32>) -> Vec<i32> {
    // 基线条件：为空或只包含一个元素的数组是“有序”的
    if array.len() < 2 {
        return array;
    } else {
        let pivot = array.remove(0); // 递归条件
                                     // 前者为由所有小于基准值的元素组成的子数组
                                     // 后者为由所有大于基准值的元素组成的子数组
        let (less, greater): (Vec<i32>, Vec<i32>) = array.into_iter().partition(|&x| x <= pivot);
        let mut sorted_less = quick_sort(less);
        let sorted_greater = quick_sort(greater);
        sorted_less.push(pivot);
        sorted_less.extend(sorted_greater);
        return sorted_less;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn case() {
        use super::*;
        let array = vec![1, 4, 4, 6, 5, 2];
        assert_eq!(quick_sort(array), vec![1, 2, 4, 4, 5, 6]);
    }
}
