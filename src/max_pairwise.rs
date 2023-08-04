use rand::prelude::*;

/// A function to find the max pairwise product in an array of integers.
/// The function taken in an array (vector) of intergers,
/// and returns the maximum product of two integers in the array.
/// Time Complexity -> O(n)
pub fn max_pairwise_product_faster(numbers: &[i64]) -> i64 {
    // Getting the first and second numbers of the array
    let first_largest_number = numbers.first().copied().unwrap();
    let second_largest_number = numbers.get(1).copied().unwrap();

    // Swap between the first and second based on which is larger
    let (mut first_largest_number, mut second_largest_number) =
        if first_largest_number > second_largest_number {
            (first_largest_number, second_largest_number)
        } else {
            (second_largest_number, first_largest_number)
        };

    // Iterate throught the remaining numbers in the array
    for &current_number in &numbers[2..] {
        // If the number is less than the second largest number, do nothing.
        if current_number <= second_largest_number {
            continue;
        }

        // If the number is less than the first largest number,
        // we assume that it is larger than the second, so we assign it to the second largest number.
        // This assumption is then verified in the first check.
        if current_number <= first_largest_number {
            second_largest_number = current_number;
            continue;
        }

        // if the number doesn't match the above conditions,
        // the number is the largest number.
        // We assign the first largest number to the second largest number,
        // and then the current number to the first largest number.
        second_largest_number = first_largest_number;
        first_largest_number = current_number;
    }
    first_largest_number * second_largest_number
}

/// A function to return the max pairwise product of an array of integers.
/// The function takes in an array of integers and returns the maximum product possible between two integers.
/// Time complexity -> O(n^2)
pub fn max_pairwise_product_naive(numbers: &[i64]) -> i64 {
    // Initialize a mutable product variable at zero
    let mut max_product: i64 = 0;

    // Iterate through the numbers starting from the first number
    for i in 0..numbers.len() {
        // Iterate through the numbers starting from i+1
        for j in i + 1..numbers.len() {
            // Compute the product of the two numbers
            let res = numbers[i] * numbers[j];
            // Check if the result is less than the current max product
            if max_product < res {
                // Assign the result to the max product
                max_product = res;
            }
        }
    }

    max_product
}

/// A function to return the max pairwise product of an array of integers using the sort function.
/// Time complexity -> should be equal to the time complexity of the sort fuction.
pub fn max_pairwise_product_by_sorting(numbers: &[i64]) -> i64 {
    // Get the max index of the array
    let n = numbers.len() - 1;
    // Convert the numbers array to a vector
    let mut a = numbers.to_vec();
    // Sort the vector in ascending order
    a.sort();

    // Multiply the last and second to last number in the array.
    a[n - 1] * a[n]
}

pub fn max_pairwise_product_fast(numbers: &[i64]) -> i64 {
    let n = numbers.len() - 1; // this is to ensure zero based indexing
    let mut a: Vec<i64> = numbers.to_vec();
    let mut idx = 0;

    for i in 0..n {
        if a[i] > a[idx] {
            idx = i;
        }
    }
    a.swap(idx, n);

    idx = 0;
    for i in 0..n {
        if a[i] > a[idx] {
            idx = i;
        }
    }
    a.swap(idx, n - 1);

    a[n - 1] * a[n]
}

#[cfg(test)]
mod tests {
    use super::max_pairwise_product_by_sorting;
    use super::max_pairwise_product_fast;
    use super::max_pairwise_product_faster;
    use super::max_pairwise_product_naive;
}
