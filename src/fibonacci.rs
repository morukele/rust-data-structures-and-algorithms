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
    let mut f: Vec<i64> = vec![0; n];
    f[0] = 0;
    f[1] = 1;

    for i in 2..=n - 1 {
        f[i] = (f[i - 1] + f[i - 2]) % 10;
    }

    f.last().unwrap().to_owned()
}

pub fn nth_fibonacci_number_modolo_m(n: i64, m: i64) -> i64 {
    let (length, pisano_sequence) = get_pisano_period_and_length(m);

    println!("pisano period lenght: {}", length);

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
            // This is a less eligant way to do it.
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

    // get the pisano period of modulus 10
    // we use 10 because we are interested in the last digit
    let (length, period) = get_pisano_period_and_length(10);
    println!("{} and {:?}", length, period);
    todo!()
}
