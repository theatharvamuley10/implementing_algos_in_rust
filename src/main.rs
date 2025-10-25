mod binary_search;
mod quicksort;
fn main() {
    let mut arr: Vec<i32> = vec![42, 7, 88, 63, 19, 54, 99, 32, 61, 5];
    println!("================\n Quicksort \n================");
    println!("Array before sorting: {:?}", arr);
    arr = quicksort::quicksort(arr);
    println!("Array after sorting: {:?}", arr);

    println!("================\n Binary Search \n================");
    let target = 63;
    println!("Sorted Array: {:?}", arr);
    println!("Target: {}", target);
    println!(
        "Index of the target is: {}",
        binary_search::binary_search(&arr, target)
    );
}
