use rand::prelude::*;

pub fn max_pairwise_product_faster(a: &Vec<i64>) -> i64 {
    //TODO: understand this implementation
    todo!()
}

pub fn max_pairwise_product_naive(a: &Vec<i64>) -> i64 {
    let mut product: i64 = 0;
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            let res = a[i] * a[j];
            if product < res {
                product = res;
            }
        }
    }

    product
}

pub fn max_pairwise_product_fast(a: &Vec<i64>) -> i64 {
    // To find the largest pairwise product, we need the largest and second largest numbers in the vector
    let mut idx_1 = 0; // Index for the first largest number

    // Find the largest number, we start from second element because the first elements is already denoted by idx_1
    for i in 1..a.len() {
        if a[i] > a[idx_1] {
            idx_1 = i;
        }
    }

    let mut idx_2; // Index for the second largest number
    if idx_1 == 0 {
        idx_2 = 1;
    } else {
        idx_2 = 0;
    }

    // Find the second largest number from the remainder of the numbers i.e sans number in idx_1
    for i in 0..a.len() {
        if i != idx_1 && a[i] > a[idx_2] {
            idx_2 = i
        }
    }

    // println!("{}, {}", idx_1, idx_2);

    a[idx_1] * a[idx_2]
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

        for i in 0..capacity {
            let num = rng.gen_range(0..=m);
            array[i] = num;
        }

        println!("{:?}", array);
        let res_1 = max_pairwise_product_naive(&array);
        let res_2 = max_pairwise_product_faster(&array);

        if res_1 == res_2 {
            println!("OK")
        } else {
            println!("Wrong anwser -> naive: {}, fast: {}", res_1, res_2);
            break;
        }
    }
}
