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

pub fn max_pairwise_product_naive(numbers: &[i64]) -> i64 {
    let mut product: i64 = 0;
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let res = numbers[i] * numbers[j];
            if product < res {
                product = res;
            }
        }
    }

    product
}

pub fn max_pairwise_product_by_sorting(numbers: &[i64]) -> i64 {
    let n = numbers.len() - 1;
    let mut a = numbers.to_vec();
    a.sort();

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

pub fn generate_large_vector(limit: i64) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::new();
    for i in 1..limit + 1 {
        res.push(i);
    }

    res
}

pub fn stress_test(n: i64, m: i64) {
    loop {
        let mut rng = thread_rng();
        let capacity = rng.gen_range(2..=n) as usize;
        let mut array: Vec<i64> = vec![0; capacity];
        (0..capacity).for_each(|i| {
            let num = rng.gen_range(0..=m);
            array[i] = num;
        });

        println!("input array: {:?}", array);
        let res_1 = max_pairwise_product_naive(&array);
        let res_2 = max_pairwise_product_by_sorting(&array);

        if res_1 == res_2 {
            println!("OK")
        } else {
            println!("Wrong answer -> naive: {}, fast: {}", res_1, res_2);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::max_pairwise_product_by_sorting;
    use super::max_pairwise_product_fast;
    use super::max_pairwise_product_faster;
    use super::max_pairwise_product_naive;
}
