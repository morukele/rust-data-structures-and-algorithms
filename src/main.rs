use puzzle::fibonacci::{
    last_digit_of_the_partial_sum_of_nth_fibonacci_number,
    last_digit_of_the_sum_of_nth_fibonacci_number,
};

fn main() {
    let res = last_digit_of_the_partial_sum_of_nth_fibonacci_number(10, 10);
    println!("output: {:?}", res);
}
