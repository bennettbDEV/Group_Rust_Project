// Mergesort, which will not implement any data structures of its own but will use vectors from the
// Rust standard library. It will use macros to demonstrate that feature of the language.
// Brett Wallin

//Performs merge sort on a mutable vector
fn mergesort(arr: &mut Vec<i32>) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        mergesort(&mut left);
        mergesort(&mut right);

        merge(arr, left, right);
    }
}

//Merges to sorted vectors into a single sorted vector
fn merge(sorted_arr: &mut Vec<i32>, left_arr: Vec<i32>, right_arr: Vec<i32>) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut sorted_arr_index = 0;

    while left_index < left_arr.len() && right_index < right_arr.len() {
        if left_arr[left_index] <= right_arr[right_index] {
            sorted_arr[sorted_arr_index] = left_arr[left_index];
            left_index += 1;
        } else {
            sorted_arr[sorted_arr_index] = right_arr[right_index];
            right_index += 1;
        }
        sorted_arr_index += 1;
    }

    while left_index < left_arr.len() {
        sorted_arr[sorted_arr_index] = left_arr[left_index];
        left_index += 1;
        sorted_arr_index += 1;
    }

    while right_index < right_arr.len() {
        sorted_arr[sorted_arr_index] = right_arr[right_index];
        right_index += 1;
        sorted_arr_index += 1;
    }
}

//The entry point for the program
fn main() {
    let mut start_arr = vec![20, 8, 12, 4, 2, 18, 36];
    println!("The starting array: {:?}", start_arr);
    mergesort(&mut start_arr);
    println!("The sorted array: {:?}", start_arr);
}
