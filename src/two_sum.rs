use std::collections::HashMap;

/// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut candidates = HashMap::new();
    let mut results = vec![];

    for (i, n) in nums.iter().enumerate() {
        if n <= &target {
            match candidates.get(&(target - n)) {
                None => {
                    candidates.insert(n, i);
                }
                Some(index) => {
                    results.push(index.to_owned() as i32);
                    results.push(i.to_owned() as i32);
                }
            }
        }
        if !results.is_empty() {
            break;
        }
    }
    results
}
