/// Performs binary search on a sorted array
/// Returns the index of the target if found, otherwise returns -1
pub fn binary_search(arr: &Vec<i32>, target: i32) -> i32 {
    if arr.is_empty() {
        return -1;
    }

    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            return mid as i32;
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            // Prevent underflow when mid is 0
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }

    -1
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_found_in_middle() {
//         let arr = [2, 5, 8, 12, 16, 21, 27, 33, 39, 45];
//         assert_eq!(binary_search(&arr, 16), 4);
//     }

//     #[test]
//     fn test_not_found() {
//         let arr = [2, 5, 8, 12, 16, 21, 27, 33, 39, 45];
//         assert_eq!(binary_search(&arr, 100), -1);
//     }

//     #[test]
//     fn test_first_element() {
//         let arr = [2, 5, 8, 12, 16, 21, 27, 33, 39, 45];
//         assert_eq!(binary_search(&arr, 2), 0);
//     }

//     #[test]
//     fn test_last_element() {
//         let arr = [2, 5, 8, 12, 16, 21, 27, 33, 39, 45];
//         assert_eq!(binary_search(&arr, 45), 9);
//     }

//     #[test]
//     fn test_empty_array() {
//         let arr: [i32; 0] = [];
//         assert_eq!(binary_search(&arr, 5), -1);
//     }

//     #[test]
//     fn test_single_element_found() {
//         let arr = [42];
//         assert_eq!(binary_search(&arr, 42), 0);
//     }

//     #[test]
//     fn test_single_element_not_found() {
//         let arr = [42];
//         assert_eq!(binary_search(&arr, 10), -1);
//     }

//     #[test]
//     fn test_two_elements() {
//         let arr = [10, 20];
//         assert_eq!(binary_search(&arr, 10), 0);
//         assert_eq!(binary_search(&arr, 20), 1);
//         assert_eq!(binary_search(&arr, 15), -1);
//     }
// }
