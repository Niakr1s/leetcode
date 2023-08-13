use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        slower::find_sums_sorted(nums, 3, 0)
    }
}

mod slower {
    use std::collections::HashSet;

    use self::frequency::Frequency;

    pub fn find_sums_sorted(
        mut nums: Vec<i32>,
        dimension: usize,
        expected_sum: i32,
    ) -> Vec<Vec<i32>> {
        let mut freq = Frequency::from(nums.into_iter());
        _find_sums_sorted(&mut freq, dimension, expected_sum)
    }

    fn _find_sums_sorted(
        freq: &mut Frequency<i32>,
        dimension: usize,
        expected_sum: i32,
    ) -> Vec<Vec<i32>> {
        match dimension {
            0 => vec![],
            1 => freq
                .count(expected_sum)
                .map(|_| vec![vec![expected_sum]])
                .unwrap_or_default(),

            _ => {
                let mut res: HashSet<Vec<i32>> = HashSet::new();
                let keys: Vec<i32> = freq.keys().cloned().collect();

                for key in keys {
                    freq.minus(key);
                    let found = _find_sums_sorted(freq, dimension - 1, expected_sum - key);
                    freq.plus(key);

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

    mod frequency {
        use std::{collections::HashMap, hash::Hash};

        #[derive(PartialEq, Debug)]
        pub struct Frequency<K>(HashMap<K, i32>)
        where
            K: Eq + Hash;

        impl<K> Frequency<K>
        where
            K: Eq + Hash,
        {
            pub fn new() -> Self {
                Self(HashMap::new())
            }

            pub fn plus(&mut self, key: K) {
                self.0.entry(key).and_modify(|v| *v += 1).or_insert(1);
            }

            pub fn minus(&mut self, key: K) {
                let v = self.0.get_mut(&key).unwrap();
                *v -= 1;
                if *v <= 0 {
                    self.0.remove(&key);
                }
            }

            pub fn count(&self, key: K) -> Option<i32> {
                self.0.get(&key).copied()
            }

            pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, i32> {
                self.0.keys()
            }
        }

        impl<I, K> From<I> for Frequency<K>
        where
            I: Iterator<Item = K>,
            K: Eq + Hash,
        {
            fn from(value: I) -> Self {
                value.fold(Frequency::new(), |mut freq, k| {
                    freq.plus(k);
                    freq
                })
            }
        }

        #[cfg(test)]
        mod test {

            use super::*;

            #[test]
            fn freq_from_1() {
                let got: Frequency<i32> = vec![0, 1, 1, 2, 2, 2].into_iter().into();
                let expected = Frequency(HashMap::from_iter([(0, 1), (1, 2), (2, 3)]));
                assert_eq!(got, expected);
            }

            #[test]
            fn freq_minus_1() {
                let mut freq: Frequency<i32> = vec![1, 1].into_iter().into();
                freq.minus(1);
                assert_eq!(Some(1), freq.count(1));
                freq.minus(1);
                assert_eq!(None, freq.count(1));
            }
        }
    }
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

    use super::Solution;

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
