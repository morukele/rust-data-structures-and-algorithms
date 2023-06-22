pub fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn fibonacci_last_digit(n: i64) -> i64 {
    // n in this case is zero based, i.e. if n is 3,
    // there should be 4 elements in the array.
    // essentially, the limit should be n+1
    let n = (n + 1) as usize;
    let mut f: Vec<i64> = vec![0; n];
    f[0] = 0;
    f[1] = 1;

    for i in 2..=n - 1 {
        f[i] = (f[i - 1] + f[i - 2]).rem_euclid(10);
    }

    f.last().unwrap().to_owned()
}
