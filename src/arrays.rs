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

pub fn group_anagrams_2(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut ans: HashMap<[i32; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut count = [0; 26];
        for c in s.chars() {
            count[c.to_ascii_lowercase() as u8 as usize
                - 'a'.to_ascii_lowercase() as u8 as usize] += 1;
        }
        ans.entry(count)
            .and_modify(|v| v.push(s.clone()))
            .or_insert(vec![s.clone()]);
    }

    let mut res: Vec<Vec<String>> = ans.into_values().collect();
    res.sort_by_key(|v| v.len());

    res
}

pub fn two_sums_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i != j && nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![] // no number in the vec can add to target
}

pub fn two_sums_faster(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    // loop through
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        // check if the complement exist in the map
        if let Some(&j) = map.get(&complement) {
            return vec![i as i32, j];
        }

        // store the number and its index in the map
        map.insert(num, i as i32);
    }

    vec![] // no number in the vec can add to target
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

    #[test]
    fn group_anagrams_2_test_1() {
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
    fn group_anagrams_2_test_2() {
        let strs = [""].iter().map(|s| s.to_string()).collect();
        let result = group_anagrams(strs);
        let output: Vec<Vec<String>> = [vec![""]]
            .iter()
            .map(|a| a.to_vec().iter().map(|s| s.to_string()).collect())
            .collect();
        assert_eq!(result, output)
    }

    #[test]
    fn group_anagrams_2_test_3() {
        let strs = ["a"].iter().map(|s| s.to_string()).collect();
        let result = group_anagrams(strs);
        let output: Vec<Vec<String>> = [vec!["a"]]
            .iter()
            .map(|a| a.to_vec().iter().map(|s| s.to_string()).collect())
            .collect();
        assert_eq!(result, output)
    }

    #[test]
    fn two_sums_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let res = two_sums_brute_force(nums, target);
        assert_eq!(res, vec![0, 1])
    }

    #[test]
    fn two_sums_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let res = two_sums_brute_force(nums, target);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn two_sums_3() {
        let nums = vec![3, 3];
        let target = 6;
        let res = two_sums_brute_force(nums, target);
        assert_eq!(res, vec![0, 1]);
    }
}
