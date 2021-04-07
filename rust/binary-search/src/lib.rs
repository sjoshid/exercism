pub fn find<T, V>(array: V, key: T) -> Option<usize>
where
    T: Ord,
    V: AsRef<[T]>,
{
    let mut result = None;
    let array = array.as_ref();
    if !array.is_empty() {
        let mut left = 0;
        let mut right = array.len() - 1;

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
