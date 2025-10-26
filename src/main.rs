mod binary_search;
mod fibonacci;
mod greedy;
mod quicksort;

use std::collections::{HashMap, HashSet};
fn main() {
    _quicksort();
    _binary_search();
    _greedy_set_cover();
    _fibonacci();
}

fn _quicksort() {
    let mut arr: Vec<i32> = vec![42, 7, 88, 63, 19, 54, 99, 32, 61, 5];
    println!("================\n Quicksort \n================");
    println!("Array before sorting: {:?}", arr);
    arr = quicksort::quicksort(arr);
    println!("Array after sorting: {:?}", arr);
}

fn _binary_search() {
    println!("\n================\n Binary Search \n================");
    let mut arr: Vec<i32> = vec![42, 7, 88, 63, 19, 54, 99, 32, 61, 5];
    arr = quicksort::quicksort(arr);
    let target = 63;
    println!("Sorted Array: {:?}", arr);
    println!("Target: {}", target);
    println!(
        "Index of the target is: {}",
        binary_search::binary_search(&arr, target)
    );
}

fn _greedy_set_cover() {
    println!("\n================\n Greedy - Set Cover Problem \n================");
    let states_needed: HashSet<&str> = ["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]
        .iter()
        .cloned()
        .collect();

    let mut stations: HashMap<&str, HashSet<&str>> = HashMap::new();
    stations.insert("kone", ["id", "nv", "ut"].iter().cloned().collect());
    stations.insert("ktwo", ["wa", "id", "mt"].iter().cloned().collect());
    stations.insert("kthree", ["or", "nv", "ca"].iter().cloned().collect());
    stations.insert("kfour", ["nv", "ut"].iter().cloned().collect());
    stations.insert("kfive", ["ca", "az"].iter().cloned().collect());

    let result = greedy::greedy_set_cover(&states_needed, &stations);

    println!("States to be covered: {:?}", states_needed);
    println!("Coverage of station one: {:?}", stations["kone"]);
    println!("Coverage of station two: {:?}", stations["ktwo"]);
    println!("Coverage of station three: {:?}", stations["kthree"]);
    println!("Coverage of station four: {:?}", stations["kfour"]);
    println!("Coverage of station five: {:?}", stations["kfive"]);
    println!("--> Selected stations: {:?}", result);
}

fn _fibonacci() {
    println!("\n================\n Fibonacci \n================");

    println!("Printing first 10 fibonacci numbers");
    fibonacci::print_fibonacci();

    let n = 5;
    println!(
        "\nThe {n}th fibonacci using recursion is: {}",
        fibonacci::find_nth_fibonacci_recursion(n)
    );
    println!(
        "The {n}th fibonacci using loop is: {}",
        fibonacci::find_nth_fibonacci_loop(n)
    );
}
