use std::{borrow::Cow, collections::HashMap};

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // sort each character of each word to get a key in alphabetical order
    // add the key to a hashmap, with the word in a corresponding vector
    // do it again, if the key exist, add the word to the vector.

    let mut ans: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs.into_iter().map(Cow::from) {
        let mut sorted = s.to_string();
        unsafe {
            sorted.as_mut_vec().sort();
        }
        ans.entry(sorted)
            .and_modify(|v| v.push(s.to_string()))
            .or_insert(vec![s.to_string()]);
    }

    let mut res: Vec<Vec<String>> = ans.into_values().collect();
    res.sort_by_key(|v| v.len());

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn group_anagrams_test_1() {
        let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = group_anagrams(strs);
        let output: Vec<Vec<String>> = [vec!["bat"], vec!["tan", "nat"], vec!["eat", "tea", "ate"]]
            .iter()
            .map(|a| a.to_vec().iter().map(|s| s.to_string()).collect())
            .collect();
        assert_eq!(result, output)
    }

    #[test]
    fn group_anagrams_test_2() {
        let strs = [""].iter().map(|s| s.to_string()).collect();
        let result = group_anagrams(strs);
        let output: Vec<Vec<String>> = [vec![""]]
            .iter()
            .map(|a| a.to_vec().iter().map(|s| s.to_string()).collect())
            .collect();
        assert_eq!(result, output)
    }

    #[test]
    fn group_anagrams_test_3() {
        let strs = ["a"].iter().map(|s| s.to_string()).collect();
        let result = group_anagrams(strs);
        let output: Vec<Vec<String>> = [vec!["a"]]
            .iter()
            .map(|a| a.to_vec().iter().map(|s| s.to_string()).collect())
            .collect();
        assert_eq!(result, output)
    }
}
