pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut result = None;
    if !array.is_empty() {
        let mut left = 0 as usize;
        let mut right = array.len() - 1 as usize;

        while left < right {
            let half = left + ((right - left) >> 1);
            if array[half] > key {
                right = half;
            } else if array[half] < key {
                left = half + 1;
            } else {
                //found
                result = Some(half);
                break;
            }
        }

        if result == None && array[left] == key {
            result = Some(left);
        }
    }

    return result;
}
