pub fn quicksort(arr: Vec<i32>) -> Vec<i32> {
    // Base case: arrays with 0 or 1 element are already sorted
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr[0];

    let mut left = Vec::new();
    let mut right = Vec::new();

    // Partition: Skip the first element (pivot), start from index 1
    for &x in arr.iter().skip(1) {
        if x <= pivot {
            left.push(x);
        } else {
            right.push(x);
        }
    }

    // Recursively sort both sides
    let mut sorted_left = quicksort(left);
    let sorted_right = quicksort(right);

    // Combine: sorted_left + pivot + sorted_right
    sorted_left.push(pivot);
    sorted_left.extend(sorted_right);

    sorted_left
}
