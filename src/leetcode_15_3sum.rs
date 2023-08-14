struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        nums.sort();
        let mut res = vec![];

        for i in 0..=nums.len() - 3 {
            // moving index forward, skiping dublicates
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            // computing sum, and if it's 0 - pushing to results
            // if sum <= 0 we are moving left idx forward, skipping dublicates
            // if sum > 0 we are moving right idx backward, skipping dublicates
            while l < r {
                let sum = nums[r] + nums[l] + nums[i];

                if sum == 0 {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                }

                if sum <= 0 {
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    l += 1;
                } else {
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    r -= 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    macro_rules! test {
        ($name:ident: $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                println!("input: {:?}", input);

                let got = sorted(Solution::three_sum(input));
                let expected: Vec<Vec<i32>> = sorted($expected);

                assert_eq!(got, expected);
            }
        };
    }

    test!(empty: vec![] => empty_vec());

    test!(one: vec![0] => empty_vec());

    test!(two: vec![0, 0] => empty_vec());

    test!(three: vec![0, 0, 0] => vec![vec![0, 0, 0]]);

    test!(three_invalid: vec![0, 1, 0] => empty_vec());

    test!(custom_1: vec![-1, 0, 1, 2, -1, -4] => vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

    test!(custom_2: vec![-1, 2, -1, 2, -1, 2, 2, 2] => vec![vec![-1, -1, 2]]);

    test!(custom_3: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] => vec![vec![0, 0, 0]]);

    fn empty_vec() -> Vec<Vec<i32>> {
        vec![]
    }

    fn sorted(v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = v
            .into_iter()
            .map(|mut v| {
                v.sort();
                v
            })
            .collect();
        v.sort();
        v
    }
}
