struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::BorrowMut, collections::HashSet};

    use super::*;

    fn to_hashset(nums: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        HashSet::from_iter(nums.into_iter().map(|mut v| {
            v.sort();
            v
        }))
    }

    macro_rules! test {
        ($name:ident: $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                let expected = to_hashset($expected);

                let got = Solution::three_sum(input);
                let got = to_hashset(got);

                assert_eq!(got, expected);
            }
        };
    }

    test!(example1: vec![-1, 0, 1, 2, -1, -4] => vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

    test!(example2: vec![0, 1, 1] => vec![]);

    test!(example3: vec![0, 0, 0] => vec![vec![0, 0, 0]]);
}
