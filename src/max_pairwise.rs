use rand::prelude::*;

pub fn max_pairwise_product_faster(a: &[i64]) -> i64 {
    let first = a.first().copied().unwrap();
    let second = a.get(1).copied().unwrap();

    let (mut first, mut second) = if first > second {
        (first, second)
    } else {
        (second, first)
    };

    for &v in &a[2..] {
        if v <= second {
            continue;
        }

        if v <= first {
            second = v;
            continue;
        }

        second = first;
        first = v;
    }
    first * second
}

pub fn max_pairwise_product_naive(a: &[i64]) -> i64 {
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

pub fn max_pairwise_product_by_sorting(a: &[i64]) -> i64 {
    let n = a.len() - 1;
    let mut a = a.to_vec();
    a.sort();

    a[n - 1] * a[n]
}

pub fn max_pairwise_product_fast(a: &[i64]) -> i64 {
    let n = a.len() - 1; // this is to ensure zero based indexing
    let mut a: Vec<i64> = a.to_vec();
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
            println!("Wrong anwser -> naive: {}, fast: {}", res_1, res_2);
            break;
        }
    }
}
