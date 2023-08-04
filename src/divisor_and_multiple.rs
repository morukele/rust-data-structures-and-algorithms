pub fn greatest_common_divisor(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;

    // This block ensures that a will always be greater than b
    // This is preferred to assert because it does not panic
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    loop {
        let res = a % b;
        if res == 0 {
            return b;
        }

        a = b;
        b = res;
    }
}

pub fn least_common_multiple(a: i64, b: i64) -> i64 {
    (a * b) / greatest_common_divisor(a, b)
}
