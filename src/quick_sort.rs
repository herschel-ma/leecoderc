pub fn quick_sort(mut array: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(array.len());
    // 基线条件：为空或只包含一个元素的数组是“有序”的
    if array.len() < 2 {
        return array;
    } else {
        let pivot = array.remove(0);
        let (less, greater): (Vec<i32>, Vec<i32>) = array.into_iter().partition(|&x| x <= pivot);
        res.append(&mut quick_sort(less));
        res.append(&mut vec![pivot]);
        res.append(&mut quick_sort(greater));
    }
    res
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
