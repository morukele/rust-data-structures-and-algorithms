pub fn nth_fibonacci_number(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    nth_fibonacci_number(n - 1) + nth_fibonacci_number(n - 2)
}

pub fn fibonacci_last_digit(n: i64) -> i64 {
    // n in this case is zero based, i.e. if n is 3,
    // there should be 4 elements in the array.
    // essentially, the limit should be n+1
    let n = (n + 1) as usize;
    let mut fib_numbers: Vec<i64> = vec![0; n];
    fib_numbers[0] = 0;
    fib_numbers[1] = 1;

    for i in 2..=n - 1 {
        fib_numbers[i] = (fib_numbers[i - 1] + fib_numbers[i - 2]) % 10;
    }

    fib_numbers.last().unwrap().to_owned()
}

pub fn nth_fibonacci_number_modulo_m(n: i64, m: i64) -> i64 {
    let (length, pisano_sequence) = get_pisano_period_and_length(m);

    println!("pisano period length: {}", length);

    let remainder = n % length;
    pisano_sequence.get(remainder as usize).unwrap().to_owned()
}

fn get_pisano_period_and_length(m: i64) -> (i64, Vec<i64>) {
    let mut a = 0;
    let mut b = 1;
    let mut lenght = 0;
    let mut pisano_sequence: Vec<i64> = vec![a, b];

    // Iterating through all the fib numbers to get the sequence
    for _i in 0..(m * m) + 1 {
        let c = (a + b) % m;

        // adding number into the sequence
        pisano_sequence.push(c);

        a = b;
        b = c;

        if a == 0 && b == 1 {
            // Remove the last two elements from the sequence
            // This is a less elegant way to do it.
            pisano_sequence.pop();
            pisano_sequence.pop();
            lenght = pisano_sequence.len() as i64;
            break;
        }
    }

    (lenght, pisano_sequence)
}

pub fn last_digit_of_the_sum_of_nth_fibonacci_number(n: i64) -> i64 {
    if n < 2 {
        return n;
    }

    // the pisano period of mod 10 is 60
    let n = ((n + 2) % 60) as usize;
    let mut fib = vec![0; n + 1];
    fib[0] = 0;
    fib[1] = 1;

    for i in 2..=n {
        fib[i] = (fib[i - 1] % 10 + fib[i - 2] % 10) % 10;
    }

    if fib[n] == 0 {
        return 9;
    }

    fib[n] % 10 - 1
}

pub fn last_digit_of_the_partial_sum_of_nth_fibonacci_number(m: i64, n: i64) -> i64 {
    if m > n {
        return 0;
    }

    // Compute the first 60 fibonacci numbers
    let mut fib: Vec<i64> = Vec::new();
    fib.push(0);
    fib.push(1);

    for i in 2..=60 {
        fib.push(fib[i - 1] + fib[i - 2]);
    }

    let m = m % 60;
    let mut n = n % 60;

    // make sure n is greater than m
    if n < m {
        n += 60;
    }

    let mut sum = 0;
    for j in m..n + 1 {
        sum += fib[(j % 60) as usize];
    }

    sum % 10
}
