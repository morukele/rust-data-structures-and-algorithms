/*
* Bubble sort is a sorting algorithm that sorts data by comparing the current item
* with the adjacent item and swaping as approperiate.
*/
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..(n - 1) {
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                // swap
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

/*
* Merge sort is a sorting algorithm that sorts data by spliting them into two halves
* reccurvisely and then sorting before merging.
*/
fn merge_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mid = n / 2;

    // handle base case
    if n < 2 {
        return; // already sorted
    }

    let (left, right) = arr.split_at_mut(mid);

    // split recurresively
    merge_sort(left);
    merge_sort(right);

    // merge both sides into the original vec
    // using temp array because of borrow checker
    let mut temp = Vec::with_capacity(left.len() + right.len());
    merge_in_place(left, right, &mut temp);

    arr.copy_from_slice(&temp); // copy from the temp array
}

fn merge_in_place(left: &mut [i32], right: &mut [i32], temp: &mut Vec<i32>) {
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] > right[j] {
            // put right first
            temp.push(right[j]);
            j += 1; // move j pointer forward
        } else {
            temp.push(left[i]);
            i += 1; // move i pointer forward
        }
    }

    // check for remaining
    while i < left.len() {
        temp.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        temp.push(right[j]);
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bubble_sort() {
        let mut arr = vec![2, 70, 32, 86, 24];
        super::bubble_sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 32, 70, 86]);
    }

    #[test]
    fn merge_sort() {
        let mut arr = vec![2, 70, 32, 86, 24];
        super::merge_sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 32, 70, 86]);
    }
}
