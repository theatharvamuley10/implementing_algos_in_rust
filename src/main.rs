mod binary_search;
fn main() {
    println!("================\n Binary Search \n================");
    let arr = [0, 1, 12, 36, 48, 50, 64, 72, 81, 900];
    let target = 50;
    println!("Sorted Array: {:?}", arr);
    println!("Target: {}", target);
    println!(
        "Index of the target is: {}",
        binary_search::binary_search(&arr, target)
    );
}
