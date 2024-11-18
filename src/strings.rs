pub fn split_by_delimiter(input: &str, delimiter: char) -> Vec<String> {
    // split input into chars
    // initialize counter for start and end
    // move counter as required
    let chars: Vec<char> = input.chars().collect();
    let mut start = 0;
    let mut end = 0;
    let mut res = vec![];

    for c in &chars[start..] {
        if c == &delimiter {
            let sub_string = &input[start..end];
            res.push(sub_string.to_string()); // add to array
            start = end + 1;
        }

        end += 1;
    }

    // check for case with no delimiter, i.e. edge case
    res.push(input[start..end].to_string());

    res
}

pub fn split_by_delimiter_idomatic(input: &str, delimiter: char) -> Vec<String> {
    let mut res = Vec::new();
    let mut current_sub_string = String::new();

    for c in input.chars() {
        if c == delimiter {
            res.push(current_sub_string); // add the current segment to the result
            current_sub_string = String::new(); // start a new segment
        } else {
            current_sub_string.push(c);
        }
    }

    // push the last segment if there is anything left
    if !current_sub_string.is_empty() {
        res.push(current_sub_string);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_by_delimiter_1() {
        let input = "ab,cd,ef";
        let result = split_by_delimiter_idomatic(input, ',');
        let expected: Vec<String> = input.split(',').map(|s| s.to_string()).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn split_by_delimiter_2() {
        let input = "abc";
        let result = split_by_delimiter_idomatic(input, ',');
        let expected: Vec<String> = input.split(',').map(|s| s.to_string()).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn split_by_delimiter_3() {
        let input = "abc&efg&hij";
        let result = split_by_delimiter_idomatic(input, '&');
        let expected: Vec<String> = input.split('&').map(|s| s.to_string()).collect();

        assert_eq!(result, expected);
    }
}
