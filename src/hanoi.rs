pub fn hanoi(n: usize, start: usize, end: usize) {
    if n == 1 {
        print_path(start, end);
    } else {
        let other = 6 - (start + end);
        hanoi(n - 1, start, other);
        print_path(start, end);
        hanoi(n - 1, other, end);
    }
}

fn print_path(start: usize, end: usize) {
    println!("{} -> {}", start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        hanoi(5, 1, 3);
    }
}
