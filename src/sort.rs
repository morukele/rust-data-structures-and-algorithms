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

#[cfg(test)]
mod tests {
    #[test]
    fn bubble_sort() {
        let mut arr = vec![2, 70, 32, 86, 24];
        super::bubble_sort(&mut arr);
        assert_eq!(arr, vec![2, 24, 32, 70, 86]);
    }
}
