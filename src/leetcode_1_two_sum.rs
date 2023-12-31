use std::collections::HashMap;

pub trait Solution {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32>;
}

pub struct NaiveSolution {}

impl Solution for NaiveSolution {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n1) in nums.iter().enumerate() {
            for (j, n2) in nums.iter().enumerate().skip(i + 1) {
                if n1 + n2 == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        unreachable!("problem says that input has exactly one solution")
    }
}

pub struct HashMapSolution {}

impl Solution for HashMapSolution {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (current, first) in nums.into_iter().enumerate() {
            let needed = target - first;
            if let Some(needed_idx) = seen.get(&needed) {
                return vec![*needed_idx as i32, current as i32];
            }
            seen.insert(first, current);
        }
        unreachable!("problem says that input has exactly one solution")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn do_test(s: impl Solution) {
        assert!(s.two_sum(vec![2, 7, 11, 15], 9) == vec![0, 1]);
        assert!(s.two_sum(vec![3, 2, 4], 6) == vec![1, 2]);
        assert!(s.two_sum(vec![3, 3], 6) == vec![0, 1]);
    }

    #[test]
    fn naive_test() {
        let s = NaiveSolution {};
        do_test(s);
    }

    #[test]
    fn hashmap_test() {
        let s = HashMapSolution {};
        do_test(s);
    }
}
