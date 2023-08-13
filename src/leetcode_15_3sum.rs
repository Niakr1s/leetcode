use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        find_sums_sorted(nums, 3, 0)
    }
}

fn find_sums_sorted(mut nums: Vec<i32>, dimension: usize, expected_sum: i32) -> Vec<Vec<i32>> {
    let mut freq = nums_to_frequency(nums);
    _find_sums_sorted(&mut freq, dimension, expected_sum)
}

fn _find_sums_sorted(
    freq: &mut HashMap<i32, i32>,
    dimension: usize,
    expected_sum: i32,
) -> Vec<Vec<i32>> {
    match dimension {
        0 => vec![],
        1 => match freq.contains_key(&expected_sum) {
            true => vec![vec![expected_sum]],
            false => vec![],
        },
        
        _ => {
            let mut res: HashSet<Vec<i32>> = HashSet::new();
            let keys: Vec<i32> = freq.keys().cloned().collect();

            for key in keys {
                freq_minus(freq, key);
                let found = _find_sums_sorted(freq, dimension - 1, expected_sum - key);
                freq_plus(freq, key);

                let mut found_with_first: Vec<Vec<i32>> = found
                    .into_iter()
                    .map(|mut sums| {
                        sums.push(key);
                        sums
                    })
                    .collect();
                for mut found in found_with_first {
                    found.sort();
                    res.insert(found);
                }
            }
            res.into_iter().collect()
        }
    }
}

fn freq_minus(freq: &mut HashMap<i32, i32>, k: i32) {
    let v = freq.get_mut(&k).unwrap();
    *v -= 1;
    if *v <= 0 {
        freq.remove(&k);
    }
}

fn freq_plus(freq: &mut HashMap<i32, i32>, k: i32) {
    freq.entry(k).and_modify(|v| *v += 1).or_insert(1);
}

fn nums_to_frequency(nums: Vec<i32>) -> HashMap<i32, i32> {
    nums.into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut freq, k| {
            freq_plus(&mut freq, k);
            freq
        })
}

mod slow {
    use std::collections::HashSet;

    pub fn find_sums_sorted(
        mut nums: Vec<i32>,
        dimension: usize,
        expected_sum: i32,
    ) -> Vec<Vec<i32>> {
        nums.sort();
        _find_sums_sorted(&nums, dimension, expected_sum).unwrap_or_default()
    }

    fn _find_sums_sorted(
        nums: &[i32],
        dimension: usize,
        expected_sum: i32,
    ) -> Option<Vec<Vec<i32>>> {
        match dimension {
            0 => None,
            _ if nums.len() < dimension => None,

            1 => nums
                .binary_search(&expected_sum)
                .ok()
                .map(|_| vec![vec![expected_sum]]),

            _ => {
                let mut res: HashSet<Vec<i32>> = HashSet::new();

                for i in 0..nums.len() {
                    let first = nums[i];
                    let remaining = &nums[i + 1..];

                    let found: Option<Vec<Vec<i32>>> =
                        _find_sums_sorted(remaining, dimension - 1, expected_sum - first);

                    if let Some(found) = found {
                        let mut found_with_first: Vec<Vec<i32>> = found
                            .into_iter()
                            .map(|mut sums| {
                                sums.push(first);
                                sums
                            })
                            .collect();
                        for mut found in found_with_first {
                            found.sort();
                            res.insert(found);
                        }
                    }
                }

                match res.is_empty() {
                    false => Some(res.into_iter().collect()),
                    true => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    macro_rules! test {
        ($name:ident: $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let expected: Vec<Vec<i32>> = sorted($expected);
                let got = sorted(Solution::three_sum($input));
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

    test!(custom_2: vec![-1, -1, 2, 2, 2] => vec![vec![-1, -1, 2]]);

    test!(custom_3: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] => vec![vec![0, 0, 0]]);

    #[test]
    fn nums_to_frequency_1() {
        let expected = HashMap::from_iter([(0, 1), (1, 2), (2, 3)].into_iter());
        let got = nums_to_frequency(vec![0, 1, 1, 2, 2, 2]);
        assert_eq!(got, expected);
    }

    #[test]
    fn freq_minus_1() {
        let mut freq: HashMap<i32, i32> = HashMap::from_iter([(1, 2)]);
        freq_minus(&mut freq, 1);
        assert_eq!(1, *freq.get(&1).unwrap());
        freq_minus(&mut freq, 1);
        assert_eq!(None, freq.get(&1));
    }

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
