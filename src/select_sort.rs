pub fn find_smallest(array: &[i32]) -> i32 {
    // smallest value
    let mut smallest = array[0];
    // index of smallest element
    let mut smallest_index = 0;

    for (i, _ele) in array.iter().enumerate() {
        if array[i] <= smallest {
            smallest = array[i];
            smallest_index = i;
        }
    }
    smallest_index as i32
}

pub fn select_sort(array: &mut Vec<i32>) -> Vec<i32> {
    let mut new_array = Vec::with_capacity(array.len());
    for _i in 0..array.len() {
        let smallest_index = find_smallest(array);
        new_array.push(array.remove(smallest_index as usize));
    }
    new_array
}

#[cfg(test)]
mod tests {
    #[test]
    fn case() {
        use super::select_sort;
        let res = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut array = vec![1, 2, 4, 5, 6, 3, 9, 8, 7];
        assert_eq!(select_sort(&mut array), res);
    }
}
